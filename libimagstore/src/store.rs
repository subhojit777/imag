//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::collections::HashMap;
use std::ops::Drop;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::sync::Arc;
use std::sync::RwLock;
use std::io::Read;
use std::convert::From;
use std::convert::Into;
use std::ops::Deref;
use std::ops::DerefMut;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Error as FMTError;

use toml::Value;
use regex::Regex;
use glob::glob;
use walkdir::WalkDir;
use walkdir::Iter as WalkDirIter;

use error::{StoreError as SE, StoreErrorKind as SEK};
use error::MapErrInto;
use storeid::{IntoStoreId, StoreId, StoreIdIterator};
use file_abstraction::FileAbstractionInstance;
use toml_ext::*;
// We re-export the following things so tests can use them
pub use file_abstraction::FileAbstraction;
pub use file_abstraction::FSFileAbstraction;
pub use file_abstraction::InMemoryFileAbstraction;

use libimagerror::into::IntoError;
use libimagerror::trace::trace_error;
use libimagutil::debug_result::*;

use self::glob_store_iter::*;

/// The Result Type returned by any interaction with the store that could fail
pub type Result<T> = RResult<T, SE>;


#[derive(Debug, PartialEq)]
enum StoreEntryStatus {
    Present,
    Borrowed
}

/// A store entry, depending on the option type it is either borrowed currently
/// or not.
#[derive(Debug)]
struct StoreEntry {
    id: StoreId,
    file: Box<FileAbstractionInstance>,
    status: StoreEntryStatus,
}

pub enum StoreObject {
    Id(StoreId),
    Collection(PathBuf),
}

pub struct Walk {
    store_path: PathBuf,
    dirwalker: WalkDirIter,
}

impl Walk {

    fn new(mut store_path: PathBuf, mod_name: &str) -> Walk {
        let pb = store_path.clone();
        store_path.push(mod_name);
        Walk {
            store_path: pb,
            dirwalker: WalkDir::new(store_path).into_iter(),
        }
    }
}

impl ::std::ops::Deref for Walk {
    type Target = WalkDirIter;

    fn deref(&self) -> &Self::Target {
        &self.dirwalker
    }
}

impl Iterator for Walk {
    type Item = StoreObject;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(something) = self.dirwalker.next() {
            match something {
                Ok(next) => if next.file_type().is_dir() {
                                return Some(StoreObject::Collection(next.path().to_path_buf()))
                            } else if next.file_type().is_file() {
                                let n   = next.path().to_path_buf();
                                let sid = match StoreId::new(Some(self.store_path.clone()), n) {
                                    Err(e) => {
                                        trace_error(&e);
                                        continue;
                                    },
                                    Ok(o) => o,
                                };
                                return Some(StoreObject::Id(sid))
                            },
                Err(e) => {
                    warn!("Error in Walker");
                    debug!("{:?}", e);
                    return None;
                }
            }
        }

        return None;
    }
}

impl StoreEntry {

    fn new(id: StoreId, backend: &Box<FileAbstraction>) -> Result<StoreEntry> {
        let pb = try!(id.clone().into_pathbuf());

        #[cfg(feature = "fs-lock")]
        {
            try!(open_file(pb.clone())
                .and_then(|f| f.lock_exclusive().map_err_into(SEK::FileError))
                .map_err_into(SEK::IoError));
        }

        Ok(StoreEntry {
            id: id,
            file: backend.new_instance(pb),
            status: StoreEntryStatus::Present,
        })
    }

    /// The entry is currently borrowed, meaning that some thread is currently
    /// mutating it
    fn is_borrowed(&self) -> bool {
        self.status == StoreEntryStatus::Borrowed
    }

    fn get_entry(&mut self) -> Result<Entry> {
        let id = &self.id.clone();
        if !self.is_borrowed() {
            self.file
                .get_file_content()
                .and_then(|content| Entry::from_str(id.clone(), &content))
                .or_else(|err| if err.err_type() == SEK::FileNotFound {
                    Ok(Entry::new(id.clone()))
                } else {
                    Err(err)
                })
        } else {
            Err(SE::new(SEK::EntryAlreadyBorrowed, None))
        }
    }

    fn write_entry(&mut self, entry: &Entry) -> Result<()> {
        if self.is_borrowed() {
            assert_eq!(self.id, entry.location);
            self.file.write_file_content(entry.to_str().as_bytes())
                .map_err_into(SEK::FileError)
                .map(|_| ())
        } else {
            Ok(())
        }
    }
}

#[cfg(feature = "fs-lock")]
impl Drop for StoreEntry {

    fn drop(self) {
        self.get_entry()
            .and_then(|entry| open_file(entry.get_location().clone()).map_err_into(SEK::IoError))
            .and_then(|f| f.unlock().map_err_into(SEK::FileError))
            .map_err_into(SEK::IoError)
    }

}


/// The Store itself, through this object one can interact with IMAG's entries
pub struct Store {
    location: PathBuf,

    ///
    /// Configuration object of the store
    ///
    configuration: Option<Value>,

    ///
    /// Internal Path->File cache map
    ///
    /// Caches the files, so they remain flock()ed
    ///
    /// Could be optimized for a threadsafe HashMap
    ///
    entries: Arc<RwLock<HashMap<StoreId, StoreEntry>>>,

    /// The backend to use
    ///
    /// This provides the filesystem-operation functions (or pretends to)
    backend: Box<FileAbstraction>,
}

impl Store {

    /// Create a new Store object
    ///
    /// This opens a Store in `location` using the configuration from `store_config` (if absent, it
    /// uses defaults).
    ///
    /// If the configuration is not valid, this fails.
    ///
    /// If the location does not exist, creating directories is by default denied and the operation
    /// fails, if not configured otherwise.
    /// An error is returned in this case.
    ///
    /// If the path exists and is a file, the operation is aborted as well, an error is returned.
    ///
    /// # Return values
    ///
    /// - On success: Store object
    /// - On Failure:
    ///   - ConfigurationError if config is faulty
    ///   - IoError(FileError(CreateStoreDirDenied())) if store location does not exist and creating
    ///     is denied
    ///   - StorePathCreate(_) if creating the store directory failed
    ///   - StorePathExists() if location exists but is a file
    pub fn new(location: PathBuf, store_config: Option<Value>) -> Result<Store> {
        let backend = Box::new(FSFileAbstraction::new());
        Store::new_with_backend(location, store_config, backend)
    }

    /// Create a Store object as descripbed in `Store::new()` documentation, but with an alternative
    /// backend implementation.
    ///
    /// Do not use directly, only for testing purposes.
    pub fn new_with_backend(location: PathBuf,
                            store_config: Option<Value>,
                            backend: Box<FileAbstraction>) -> Result<Store> {
        use configuration::*;

        debug!("Validating Store configuration");
        let _ = try!(config_is_valid(&store_config).map_err_into(SEK::ConfigurationError));

        debug!("Building new Store object");
        if !location.exists() {
            if !config_implicit_store_create_allowed(store_config.as_ref()) {
                warn!("Implicitely creating store directory is denied");
                warn!(" -> Either because configuration does not allow it");
                warn!(" -> or because there is no configuration");
                return Err(SEK::CreateStoreDirDenied.into_error())
                    .map_err_into(SEK::FileError)
                    .map_err_into(SEK::IoError);
            }

            try!(backend.create_dir_all(&location)
                 .map_err_into(SEK::StorePathCreate)
                 .map_dbg_err_str("Failed"));
        } else if location.is_file() {
            debug!("Store path exists as file");
            return Err(SEK::StorePathExists.into_error());
        }

        let store = Store {
            location: location.clone(),
            configuration: store_config,
            entries: Arc::new(RwLock::new(HashMap::new())),
            backend: backend,
        };

        debug!("Store building succeeded");
        debug!("------------------------");
        debug!("{:?}", store);
        debug!("------------------------");

        Ok(store)
    }

    /// Get the store configuration
    pub fn config(&self) -> Option<&Value> {
        self.configuration.as_ref()
    }

    /// Verify the store.
    ///
    /// This function is not intended to be called by normal programs but only by `imag-store`.
    #[cfg(feature = "verify")]
    pub fn verify(&self) -> bool {
        use libimagerror::trace::trace_error_dbg;

        info!("Header | Content length | Path");
        info!("-------+----------------+-----");

        WalkDir::new(self.location.clone())
            .into_iter()
            .all(|res| match res {
                Ok(dent) => {
                    if dent.file_type().is_file() {
                        match self.get(PathBuf::from(dent.path())) {
                            Ok(Some(fle)) => {
                                let p           = fle.get_location();
                                let content_len = fle.get_content().len();
                                let header      = if fle.get_header().verify().is_ok() {
                                    "ok"
                                } else {
                                    "broken"
                                };

                                info!("{: >6} | {: >14} | {:?}", header, content_len, p.deref());
                                true
                            },

                            Ok(None) => {
                                info!("{: >6} | {: >14} | {:?}", "?", "couldn't load", dent.path());
                                true
                            },

                            Err(e) => {
                                trace_error_dbg(&e);
                                if_cfg_panic!("Error verifying: {:?}", e);
                                debug!("{:?}", e);
                                false
                            },
                        }
                    } else {
                        info!("{: >6} | {: >14} | {:?}", "?", "<no file>", dent.path());
                        true
                    }
                },

                Err(e) => {
                    trace_error_dbg(&e);
                    if_cfg_panic!("Error verifying: {:?}", e);
                    debug!("{:?}", e);
                    false
                },
            })
    }

    /// Creates the Entry at the given location (inside the entry)
    ///
    /// # Return value
    ///
    /// On success: FileLockEntry
    ///
    /// On error:
    ///  - Errors StoreId::into_storeid() might return
    ///  - CreateCallError(LockPoisoned()) if the internal lock is poisened.
    ///  - CreateCallError(EntryAlreadyExists()) if the entry exists already.
    ///
    pub fn create<'a, S: IntoStoreId>(&'a self, id: S) -> Result<FileLockEntry<'a>> {
        let id = try!(id.into_storeid()).with_base(self.path().clone());

        debug!("Creating id: '{}'", id);

        {
            let mut hsmap = match self.entries.write() {
                Err(_) => return Err(SEK::LockPoisoned.into_error()).map_err_into(SEK::CreateCallError),
                Ok(s) => s,
            };

            if hsmap.contains_key(&id) {
                debug!("Cannot create, internal cache already contains: '{}'", id);
                return Err(SEK::EntryAlreadyExists.into_error()).map_err_into(SEK::CreateCallError);
            }
            hsmap.insert(id.clone(), {
                debug!("Creating: '{}'", id);
                let mut se = try!(StoreEntry::new(id.clone(), &self.backend));
                se.status = StoreEntryStatus::Borrowed;
                se
            });
        }

        debug!("Constructing FileLockEntry: '{}'", id);

        Ok(FileLockEntry::new(self, Entry::new(id)))
    }

    /// Borrow a given Entry. When the `FileLockEntry` is either `update`d or
    /// dropped, the new Entry is written to disk
    ///
    /// Implicitely creates a entry in the store if there is no entry with the id `id`. For a
    /// non-implicitely-create look at `Store::get`.
    ///
    /// # Return value
    ///
    /// On success: FileLockEntry
    ///
    /// On error:
    ///  - Errors StoreId::into_storeid() might return
    ///  - RetrieveCallError(LockPoisoned()) if the internal lock is poisened.
    ///
    pub fn retrieve<'a, S: IntoStoreId>(&'a self, id: S) -> Result<FileLockEntry<'a>> {
        let id = try!(id.into_storeid()).with_base(self.path().clone());
        debug!("Retrieving id: '{}'", id);
        let entry = try!({
            self.entries
                .write()
                .map_err(|_| SE::new(SEK::LockPoisoned, None))
                .and_then(|mut es| {
                    let new_se = try!(StoreEntry::new(id.clone(), &self.backend));
                    let mut se = es.entry(id.clone()).or_insert(new_se);
                    let entry = se.get_entry();
                    se.status = StoreEntryStatus::Borrowed;
                    entry
                })
                .map_err_into(SEK::RetrieveCallError)
        });

        debug!("Constructing FileLockEntry: '{}'", id);
        Ok(FileLockEntry::new(self, entry))
    }

    /// Get an entry from the store if it exists.
    ///
    /// # Return value
    ///
    /// On success: Some(FileLockEntry) or None
    ///
    /// On error:
    ///  - Errors StoreId::into_storeid() might return
    ///  - Errors Store::retrieve() might return
    ///
    pub fn get<'a, S: IntoStoreId + Clone>(&'a self, id: S) -> Result<Option<FileLockEntry<'a>>> {
        let id = try!(id.into_storeid()).with_base(self.path().clone());

        debug!("Getting id: '{}'", id);

        let exists = try!(id.exists()) || try!(self.entries
            .read()
            .map(|map| map.contains_key(&id))
            .map_err(|_| SE::new(SEK::LockPoisoned, None))
            .map_err_into(SEK::GetCallError)
        );

        if !exists {
            debug!("Does not exist in internal cache or filesystem: {:?}", id);
            return Ok(None);
        }

        self.retrieve(id).map(Some).map_err_into(SEK::GetCallError)
    }

    /// Iterate over all StoreIds for one module name
    ///
    /// # Returns
    ///
    /// On success: An iterator over all entries in the module
    ///
    /// On failure:
    ///  - RetrieveForModuleCallError(GlobError(EncodingError())) if the path string cannot be
    ///    encoded
    ///  - GRetrieveForModuleCallError(GlobError(lobError())) if the glob() failed.
    ///
    pub fn retrieve_for_module(&self, mod_name: &str) -> Result<StoreIdIterator> {
        let mut path = self.path().clone();
        path.push(mod_name);

        debug!("Retrieving for module: '{}'", mod_name);

        path.to_str()
            .ok_or(SE::new(SEK::EncodingError, None))
            .and_then(|path| {
                let path = [ path, "/**/*" ].join("");
                debug!("glob()ing with '{}'", path);
                glob(&path[..]).map_err_into(SEK::GlobError)
            })
            .map(|paths| GlobStoreIdIterator::new(paths, self.path().clone()).into())
            .map_err_into(SEK::GlobError)
            .map_err_into(SEK::RetrieveForModuleCallError)
    }

    /// Walk the store tree for the module
    ///
    /// The difference between a `Walk` and a `StoreIdIterator` is that with a `Walk`, one can find
    /// "collections" (folders).
    pub fn walk<'a>(&'a self, mod_name: &str) -> Walk {
        debug!("Creating Walk object for {}", mod_name);
        Walk::new(self.path().clone(), mod_name)
    }

    /// Return the `FileLockEntry` and write to disk
    ///
    /// See `Store::_update()`.
    ///
    pub fn update<'a>(&'a self, entry: &mut FileLockEntry<'a>) -> Result<()> {
        debug!("Updating FileLockEntry at '{}'", entry.get_location());
        self._update(entry, false).map_err_into(SEK::UpdateCallError)
    }

    /// Internal method to write to the filesystem store.
    ///
    /// # Assumptions
    ///
    /// This method assumes that entry is dropped _right after_ the call, hence
    /// it is not public.
    ///
    /// # Return value
    ///
    /// On success: Entry
    ///
    /// On error:
    ///  - UpdateCallError(LockPoisoned()) if the internal write lock cannot be aquierd.
    ///  - IdNotFound() if the entry was not found in the stor
    ///  - Errors Entry::verify() might return
    ///  - Errors StoreEntry::write_entry() might return
    ///
    fn _update<'a>(&'a self, entry: &mut FileLockEntry<'a>, modify_presence: bool) -> Result<()> {
        let mut hsmap = match self.entries.write() {
            Err(_) => return Err(SE::new(SEK::LockPoisoned, None)),
            Ok(e) => e,
        };

        let mut se = try!(hsmap.get_mut(&entry.location).ok_or(SE::new(SEK::IdNotFound, None)));

        assert!(se.is_borrowed(), "Tried to update a non borrowed entry.");

        debug!("Verifying Entry");
        try!(entry.entry.verify());

        debug!("Writing Entry");
        try!(se.write_entry(&entry.entry));
        if modify_presence {
            se.status = StoreEntryStatus::Present;
        }

        Ok(())
    }

    /// Retrieve a copy of a given entry, this cannot be used to mutate
    /// the one on disk
    ///
    /// # Return value
    ///
    /// On success: Entry
    ///
    /// On error:
    ///  - RetrieveCopyCallError(LockPoisoned()) if the internal write lock cannot be aquierd.
    ///  - RetrieveCopyCallError(IdLocked()) if the Entry is borrowed currently
    ///  - Errors StoreEntry::new() might return
    ///
    pub fn retrieve_copy<S: IntoStoreId>(&self, id: S) -> Result<Entry> {
        let id = try!(id.into_storeid()).with_base(self.path().clone());
        debug!("Retrieving copy of '{}'", id);
        let entries = match self.entries.write() {
            Err(_) => {
                return Err(SE::new(SEK::LockPoisoned, None))
                    .map_err_into(SEK::RetrieveCopyCallError);
            },
            Ok(e) => e,
        };

        // if the entry is currently modified by the user, we cannot drop it
        if entries.get(&id).map(|e| e.is_borrowed()).unwrap_or(false) {
            return Err(SE::new(SEK::IdLocked, None)).map_err_into(SEK::RetrieveCopyCallError);
        }

        try!(StoreEntry::new(id, &self.backend)).get_entry()
    }

    /// Delete an entry
    ///
    /// # Return value
    ///
    /// On success: ()
    ///
    /// On error:
    ///  - DeleteCallError(LockPoisoned()) if the internal write lock cannot be aquierd.
    ///  - DeleteCallError(FileNotFound()) if the StoreId refers to a non-existing entry.
    ///  - DeleteCallError(FileError()) if the internals failed to remove the file.
    ///
    pub fn delete<S: IntoStoreId>(&self, id: S) -> Result<()> {
        let id = try!(id.into_storeid()).with_base(self.path().clone());

        debug!("Deleting id: '{}'", id);

        {
            let mut entries = match self.entries.write() {
                Err(_) => return Err(SE::new(SEK::LockPoisoned, None))
                    .map_err_into(SEK::DeleteCallError),
                Ok(e) => e,
            };

            // if the entry is currently modified by the user, we cannot drop it
            match entries.get(&id) {
                None => {
                    return Err(SEK::FileNotFound.into_error()).map_err_into(SEK::DeleteCallError)
                },
                Some(e) => if e.is_borrowed() {
                    return Err(SE::new(SEK::IdLocked, None)).map_err_into(SEK::DeleteCallError)
                }
            }

            // remove the entry first, then the file
            entries.remove(&id);
            let pb = try!(id.clone().with_base(self.path().clone()).into_pathbuf());
            if let Err(e) = self.backend.remove_file(&pb) {
                return Err(SEK::FileError.into_error_with_cause(Box::new(e)))
                    .map_err_into(SEK::DeleteCallError);
            }
        }

        debug!("Deleted");
        Ok(())
    }

    /// Save a copy of the Entry in another place
    pub fn save_to(&self, entry: &FileLockEntry, new_id: StoreId) -> Result<()> {
        debug!("Saving '{}' to '{}'", entry.get_location(), new_id);
        self.save_to_other_location(entry, new_id, false)
    }

    /// Save an Entry in another place
    /// Removes the original entry
    pub fn save_as(&self, entry: FileLockEntry, new_id: StoreId) -> Result<()> {
        debug!("Saving '{}' as '{}'", entry.get_location(), new_id);
        self.save_to_other_location(&entry, new_id, true)
    }

    fn save_to_other_location(&self, entry: &FileLockEntry, new_id: StoreId, remove_old: bool)
        -> Result<()>
    {
        let new_id = new_id.with_base(self.path().clone());
        let hsmap = try!(
            self.entries
                .write()
                .map_err(|_| SEK::LockPoisoned.into_error())
                .map_err_into(SEK::MoveCallError)
        );

        if hsmap.contains_key(&new_id) {
            return Err(SEK::EntryAlreadyExists.into_error()).map_err_into(SEK::MoveCallError)
        }

        let old_id = entry.get_location().clone();

        let old_id_as_path = try!(old_id.clone().with_base(self.path().clone()).into_pathbuf());
        let new_id_as_path = try!(new_id.clone().with_base(self.path().clone()).into_pathbuf());
        self.backend.copy(&old_id_as_path, &new_id_as_path)
            .and_then(|_| {
                if remove_old {
                    debug!("Removing old '{:?}'", old_id_as_path);
                    self.backend.remove_file(&old_id_as_path)
                } else {
                    Ok(())
                }
            })
            .map_err_into(SEK::FileError)
            .map_err_into(SEK::MoveCallError)
    }

    /// Move an entry without loading
    ///
    /// This function moves an entry from one path to another.
    ///
    /// Generally, this function shouldn't be used by library authors, if they "just" want to move
    /// something around. A library for moving entries while caring about meta-data and links.
    ///
    /// # Errors
    ///
    /// This function returns an error in certain cases:
    ///
    /// * If the about-to-be-moved entry is borrowed
    /// * If the lock on the internal data structure cannot be aquired
    /// * If the new path already exists
    /// * If the about-to-be-moved entry does not exist
    /// * If the FS-operation failed
    ///
    /// # Warnings
    ///
    /// This should be used with _great_ care, as moving an entry from `a` to `b` might result in
    /// dangling links (see below).
    ///
    /// ## Moving linked entries
    ///
    /// If the entry which is moved is linked to another entry, these links get invalid (but we do
    /// not detect this here). As links are always two-way-links, so `a` is not only linked to `b`,
    /// but also the other way round, moving `b` to `c` results in the following scenario:
    ///
    /// * `a` links to `b`, which does not exist anymore.
    /// * `c` links to `a`, which does exist.
    ///
    /// So the link is _partly dangling_, so to say.
    ///
    pub fn move_by_id(&self, old_id: StoreId, new_id: StoreId) -> Result<()> {
        let new_id = new_id.with_base(self.path().clone());
        let old_id = old_id.with_base(self.path().clone());

        debug!("Moving '{}' to '{}'", old_id, new_id);

        {
            let mut hsmap = match self.entries.write() {
                Err(_) => return Err(SE::new(SEK::LockPoisoned, None)),
                Ok(m)  => m,
            };

            if hsmap.contains_key(&new_id) {
                return Err(SEK::EntryAlreadyExists.into_error());
            }

            // if we do not have an entry here, we fail in `FileAbstraction::rename()` below.
            // if we have one, but it is borrowed, we really should not rename it, as this might
            // lead to strange errors
            if hsmap.get(&old_id).map(|e| e.is_borrowed()).unwrap_or(false) {
                return Err(SEK::EntryAlreadyBorrowed.into_error());
            }

            let old_id_pb = try!(old_id.clone().with_base(self.path().clone()).into_pathbuf());
            let new_id_pb = try!(new_id.clone().with_base(self.path().clone()).into_pathbuf());

            match self.backend.rename(&old_id_pb, &new_id_pb) {
                Err(e) => return Err(SEK::EntryRenameError.into_error_with_cause(Box::new(e))),
                Ok(_) => {
                    debug!("Rename worked on filesystem");

                    // assert enforced through check hsmap.contains_key(&new_id) above.
                    // Should therefor never fail
                    assert!(hsmap
                            .remove(&old_id)
                            .and_then(|mut entry| {
                                entry.id = new_id.clone();
                                hsmap.insert(new_id.clone(), entry)
                            }).is_none())
                }
            }

        }

        debug!("Moved");
        Ok(())
    }

    /// Gets the path where this store is on the disk
    pub fn path(&self) -> &PathBuf {
        &self.location
    }

}

impl Debug for Store {

    /// TODO: Make pretty.
    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FMTError> {
        try!(write!(fmt, " --- Store ---\n"));
        try!(write!(fmt, "\n"));
        try!(write!(fmt, " - location               : {:?}\n", self.location));
        try!(write!(fmt, " - configuration          : {:?}\n", self.configuration));
        try!(write!(fmt, "\n"));
        try!(write!(fmt, "Entries:\n"));
        try!(write!(fmt, "{:?}", self.entries));
        try!(write!(fmt, "\n"));
        Ok(())
    }

}

impl Drop for Store {

    ///
    /// Unlock all files on drop
    //
    /// TODO: Unlock them
    ///
    fn drop(&mut self) {
        debug!("Dropping store");
    }

}

/// A struct that allows you to borrow an Entry
pub struct FileLockEntry<'a> {
    store: &'a Store,
    entry: Entry,
}

impl<'a> FileLockEntry<'a, > {

    /// Create a new FileLockEntry based on a `Entry` object.
    ///
    /// Only for internal use.
    fn new(store: &'a Store, entry: Entry) -> FileLockEntry<'a> {
        FileLockEntry {
            store: store,
            entry: entry,
        }
    }
}

impl<'a> Debug for FileLockEntry<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FMTError> {
        write!(fmt, "FileLockEntry(Store = {})", self.store.location.to_str()
               .unwrap_or("Unknown Path"))
    }
}

impl<'a> Deref for FileLockEntry<'a> {
    type Target = Entry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl<'a> DerefMut for FileLockEntry<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry
    }
}

#[cfg(not(test))]
impl<'a> Drop for FileLockEntry<'a> {

    /// This will silently ignore errors, use `Store::update` if you want to catch the errors
    ///
    /// This might panic if the store was compiled with the early-panic feature (which is not
    /// intended for production use, though).
    fn drop(&mut self) {
        use libimagerror::trace::trace_error_dbg;
        match self.store._update(self, true) {
            Err(e) => {
                trace_error_dbg(&e);
                if_cfg_panic!("ERROR WHILE DROPPING: {:?}", e);
            },
            Ok(_) => { },
        }
    }
}

#[cfg(test)]
impl<'a> Drop for FileLockEntry<'a> {

    /// This will not silently ignore errors but prints the result of the _update() call for testing
    fn drop(&mut self) {
        let _ = self.store._update(self, true).map_err(|e| trace_error(&e));
    }

}


/// `EntryContent` type
pub type EntryContent = String;

/// An Entry of the store
//
/// Contains location, header and content part.
#[derive(Debug, Clone)]
pub struct Entry {
    location: StoreId,
    header: Value,
    content: EntryContent,
}

impl Entry {

    /// Create a new store entry with its location at `loc`.
    ///
    /// This creates the entry with the default header from `Entry::default_header()` and an empty
    /// content.
    pub fn new(loc: StoreId) -> Entry {
        Entry {
            location: loc,
            header: Entry::default_header(),
            content: EntryContent::new()
        }
    }

    /// Get the default Header for an Entry.
    ///
    /// This function should be used to get a new Header, as the default header may change. Via
    /// this function, compatibility is ensured.
    pub fn default_header() -> Value { // BTreeMap<String, Value>
        Value::default_header()
    }

    /// See `Entry::from_str()`, as this function is used internally. This is just a wrapper for
    /// convenience.
    pub fn from_reader<S: IntoStoreId>(loc: S, file: &mut Read) -> Result<Entry> {
        let text = {
            let mut s = String::new();
            try!(file.read_to_string(&mut s));
            s
        };
        Self::from_str(loc, &text[..])
    }

    /// Create a new Entry, with contents from the string passed.
    ///
    /// The passed string _must_ be a complete valid store entry, including header. So this is
    /// probably not what end-users want to call.
    ///
    /// # Return value
    ///
    /// This errors if
    ///
    /// - String cannot be matched on regex to find header and content
    /// - Header cannot be parsed into a TOML object
    ///
    pub fn from_str<S: IntoStoreId>(loc: S, s: &str) -> Result<Entry> {
        debug!("Building entry from string");
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?smx)
                ^---$
                (?P<header>.*) # Header
                ^---$\n
                (?P<content>.*) # Content
            ").unwrap();
        }

        let matches = match RE.captures(s) {
            None    => return Err(SE::new(SEK::MalformedEntry, None)),
            Some(s) => s,
        };

        let header = match matches.name("header") {
            None    => return Err(SE::new(SEK::MalformedEntry, None)),
            Some(s) => s
        };

        let content = matches.name("content").map(|r| r.as_str()).unwrap_or("");

        debug!("Header and content found. Yay! Building Entry object now");
        Ok(Entry {
            location: try!(loc.into_storeid()),
            header: try!(Value::parse(header.as_str())),
            content: String::from(content),
        })
    }

    /// Return the string representation of this entry
    ///
    /// This means not only the content of the entry, but the complete entry (from memory, not from
    /// disk).
    pub fn to_str(&self) -> String {
        format!("---\n{header}---\n{content}",
                header  = ::toml::ser::to_string(&self.header).unwrap(),
                content = self.content)
    }

    /// Get the location of the Entry
    pub fn get_location(&self) -> &StoreId {
        &self.location
    }

    /// Get the header of the Entry
    pub fn get_header(&self) -> &Value {
        &self.header
    }

    /// Get the header mutably of the Entry
    pub fn get_header_mut(&mut self) -> &mut Value {
        &mut self.header
    }

    /// Get the content of the Entry
    pub fn get_content(&self) -> &EntryContent {
        &self.content
    }

    /// Get the content mutably of the Entry
    pub fn get_content_mut(&mut self) -> &mut EntryContent {
        &mut self.content
    }

    /// Verify the entry.
    ///
    /// Currently, this only verifies the header. This might change in the future.
    pub fn verify(&self) -> Result<()> {
        self.header.verify()
    }

}

impl PartialEq for Entry {

    fn eq(&self, other: &Entry) -> bool {
        self.location == other.location && // As the location only compares from the store root
            self.header == other.header && // and the other Entry could be from another store (not
            self.content == other.content  // implemented by now, but we think ahead here)
    }

}

mod glob_store_iter {
    use std::fmt::{Debug, Formatter};
    use std::fmt::Error as FmtError;
    use std::path::PathBuf;
    use glob::Paths;
    use storeid::StoreId;
    use storeid::StoreIdIterator;

    use error::StoreErrorKind as SEK;
    use error::MapErrInto;

    use libimagerror::trace::trace_error;

    pub struct GlobStoreIdIterator {
        store_path: PathBuf,
        paths: Paths,
    }

    impl Debug for GlobStoreIdIterator {

        fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
            write!(fmt, "GlobStoreIdIterator")
        }

    }

    impl Into<StoreIdIterator> for GlobStoreIdIterator {

        fn into(self) -> StoreIdIterator {
            StoreIdIterator::new(Box::new(self))
        }

    }

    impl GlobStoreIdIterator {

        pub fn new(paths: Paths, store_path: PathBuf) -> GlobStoreIdIterator {
            debug!("Create a GlobStoreIdIterator(store_path = {:?}, /* ... */)", store_path);

            GlobStoreIdIterator {
                store_path: store_path,
                paths: paths,
            }
        }

    }

    impl Iterator for GlobStoreIdIterator {
        type Item = StoreId;

        fn next(&mut self) -> Option<StoreId> {
            self.paths
                .next()
                .and_then(|o| {
                    debug!("GlobStoreIdIterator::next() => {:?}", o);
                    o.map_err_into(SEK::StoreIdHandlingError)
                        .and_then(|p| StoreId::from_full_path(&self.store_path, p))
                        .map_err(|e| {
                            debug!("GlobStoreIdIterator error: {:?}", e);
                            trace_error(&e);
                        }).ok()
                })
        }

    }

}


#[cfg(test)]
mod test {
    extern crate env_logger;

    use std::collections::BTreeMap;
    use storeid::StoreId;

    use toml::Value;

    #[test]
    fn test_imag_section() {
        use toml_ext::has_main_section;

        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Table(BTreeMap::new()));

        assert!(has_main_section(&map));
    }

    #[test]
    fn test_imag_invalid_section_type() {
        use toml_ext::has_main_section;

        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Boolean(false));

        assert!(!has_main_section(&map));
    }

    #[test]
    fn test_imag_abscent_main_section() {
        use toml_ext::has_main_section;

        let mut map = BTreeMap::new();
        map.insert("not_imag".into(), Value::Boolean(false));

        assert!(!has_main_section(&map));
    }

    #[test]
    fn test_main_section_without_version() {
        use toml_ext::has_imag_version_in_main_section;

        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Table(BTreeMap::new()));

        assert!(!has_imag_version_in_main_section(&map));
    }

    #[test]
    fn test_main_section_with_version() {
        use toml_ext::has_imag_version_in_main_section;

        let mut map = BTreeMap::new();
        let mut sub = BTreeMap::new();
        sub.insert("version".into(), Value::String("0.0.0".into()));
        map.insert("imag".into(), Value::Table(sub));

        assert!(has_imag_version_in_main_section(&map));
    }

    #[test]
    fn test_main_section_with_version_in_wrong_type() {
        use toml_ext::has_imag_version_in_main_section;

        let mut map = BTreeMap::new();
        let mut sub = BTreeMap::new();
        sub.insert("version".into(), Value::Boolean(false));
        map.insert("imag".into(), Value::Table(sub));

        assert!(!has_imag_version_in_main_section(&map));
    }

    #[test]
    fn test_verification_good() {
        use toml_ext::verify_header_consistency;

        let mut header = BTreeMap::new();
        let sub = {
            let mut sub = BTreeMap::new();
            sub.insert("version".into(), Value::String(String::from("0.0.0")));

            Value::Table(sub)
        };

        header.insert("imag".into(), sub);

        assert!(verify_header_consistency(header).is_ok());
    }

    #[test]
    fn test_verification_invalid_versionstring() {
        use toml_ext::verify_header_consistency;

        let mut header = BTreeMap::new();
        let sub = {
            let mut sub = BTreeMap::new();
            sub.insert("version".into(), Value::String(String::from("000")));

            Value::Table(sub)
        };

        header.insert("imag".into(), sub);

        assert!(!verify_header_consistency(header).is_ok());
    }


    #[test]
    fn test_verification_current_version() {
        use toml_ext::verify_header_consistency;

        let mut header = BTreeMap::new();
        let sub = {
            let mut sub = BTreeMap::new();
            sub.insert("version".into(), Value::String(String::from(version!())));

            Value::Table(sub)
        };

        header.insert("imag".into(), sub);

        assert!(verify_header_consistency(header).is_ok());
    }

    static TEST_ENTRY : &'static str = "---
[imag]
version = \"0.0.3\"
---
Hai";

    #[test]
    fn test_entry_from_str() {
        use super::Entry;
        use std::path::PathBuf;
        println!("{}", TEST_ENTRY);
        let entry = Entry::from_str(StoreId::new_baseless(PathBuf::from("test/foo~1.3")).unwrap(),
                                    TEST_ENTRY).unwrap();

        assert_eq!(entry.content, "Hai");
    }

    #[test]
    fn test_entry_to_str() {
        use super::Entry;
        use std::path::PathBuf;
        println!("{}", TEST_ENTRY);
        let entry = Entry::from_str(StoreId::new_baseless(PathBuf::from("test/foo~1.3")).unwrap(),
                                    TEST_ENTRY).unwrap();
        let string = entry.to_str();

        assert_eq!(TEST_ENTRY, string);
    }

}

#[cfg(test)]
mod store_tests {
    use std::path::PathBuf;

    use super::Store;
    use file_abstraction::InMemoryFileAbstraction;

    pub fn get_store() -> Store {
        let backend = Box::new(InMemoryFileAbstraction::new());
        Store::new_with_backend(PathBuf::from("/"), None, backend).unwrap()
    }

    #[test]
    fn test_store_instantiation() {
        let store = get_store();

        assert_eq!(store.location, PathBuf::from("/"));
        assert!(store.entries.read().unwrap().is_empty());
    }

    #[test]
    fn test_store_create() {
        let store = get_store();

        for n in 1..100 {
            let s = format!("test-{}", n);
            let entry = store.create(PathBuf::from(s.clone())).unwrap();
            assert!(entry.verify().is_ok());
            let loc = entry.get_location().clone().into_pathbuf().unwrap();
            assert!(loc.starts_with("/"));
            assert!(loc.ends_with(s));
        }
    }

    #[test]
    fn test_store_get_create_get_delete_get() {
        let store = get_store();

        for n in 1..100 {
            let res = store.get(PathBuf::from(format!("test-{}", n)));
            assert!(match res { Ok(None) => true, _ => false, })
        }

        for n in 1..100 {
            let s = format!("test-{}", n);
            let entry = store.create(PathBuf::from(s.clone())).unwrap();

            assert!(entry.verify().is_ok());

            let loc = entry.get_location().clone().into_pathbuf().unwrap();

            assert!(loc.starts_with("/"));
            assert!(loc.ends_with(s));
        }

        for n in 1..100 {
            let res = store.get(PathBuf::from(format!("test-{}", n)));
            assert!(match res { Ok(Some(_)) => true, _ => false, })
        }

        for n in 1..100 {
            assert!(store.delete(PathBuf::from(format!("test-{}", n))).is_ok())
        }

        for n in 1..100 {
            let res = store.get(PathBuf::from(format!("test-{}", n)));
            assert!(match res { Ok(None) => true, _ => false, })
        }
    }

    #[test]
    fn test_store_create_twice() {
        use error::StoreErrorKind as SEK;

        let store = get_store();

        for n in 1..100 {
            let s = format!("test-{}", n % 50);
            store.create(PathBuf::from(s.clone()))
                .map_err(|e| assert!(is_match!(e.err_type(), SEK::CreateCallError) && n >= 50))
                .ok()
                .map(|entry| {
                    assert!(entry.verify().is_ok());
                    let loc = entry.get_location().clone().into_pathbuf().unwrap();
                    assert!(loc.starts_with("/"));
                    assert!(loc.ends_with(s));
                });
        }
    }

    #[test]
    fn test_store_create_in_hm() {
        use storeid::StoreId;

        let store = get_store();

        for n in 1..100 {
            let pb = StoreId::new_baseless(PathBuf::from(format!("test-{}", n))).unwrap();

            assert!(store.entries.read().unwrap().get(&pb).is_none());
            assert!(store.create(pb.clone()).is_ok());

            let pb = pb.with_base(store.path().clone());
            assert!(store.entries.read().unwrap().get(&pb).is_some());
        }
    }

    #[test]
    fn test_store_retrieve_in_hm() {
        use storeid::StoreId;

        let store = get_store();

        for n in 1..100 {
            let pb = StoreId::new_baseless(PathBuf::from(format!("test-{}", n))).unwrap();

            assert!(store.entries.read().unwrap().get(&pb).is_none());
            assert!(store.retrieve(pb.clone()).is_ok());

            let pb = pb.with_base(store.path().clone());
            assert!(store.entries.read().unwrap().get(&pb).is_some());
        }
    }

    #[test]
    fn test_get_none() {
        let store = get_store();

        for n in 1..100 {
            match store.get(PathBuf::from(format!("test-{}", n))) {
                Ok(None) => assert!(true),
                _        => assert!(false),
            }
        }
    }

    #[test]
    fn test_delete_none() {
        let store = get_store();

        for n in 1..100 {
            match store.delete(PathBuf::from(format!("test-{}", n))) {
                Err(_) => assert!(true),
                _      => assert!(false),
            }
        }
    }

    // Disabled because we cannot test this by now, as we rely on glob() in
    // Store::retieve_for_module(), which accesses the filesystem and tests run in-memory, so there
    // are no files on the filesystem in this test after Store::create().
    //
    // #[test]
    // fn test_retrieve_for_module() {
    //     let pathes = vec![
    //         "foo/1", "foo/2", "foo/3", "foo/4", "foo/5",
    //         "bar/1", "bar/2", "bar/3", "bar/4", "bar/5",
    //         "bla/1", "bla/2", "bla/3", "bla/4", "bla/5",
    //         "boo/1", "boo/2", "boo/3", "boo/4", "boo/5",
    //         "glu/1", "glu/2", "glu/3", "glu/4", "glu/5",
    //     ];

    //     fn test(store: &Store, modulename: &str) {
    //         use std::path::Component;
    //         use storeid::StoreId;

    //         let retrieved = store.retrieve_for_module(modulename);
    //         assert!(retrieved.is_ok());
    //         let v : Vec<StoreId> = retrieved.unwrap().collect();
    //         println!("v = {:?}", v);
    //         assert!(v.len() == 5);

    //         let retrieved = store.retrieve_for_module(modulename);
    //         assert!(retrieved.is_ok());

    //         assert!(retrieved.unwrap().all(|e| {
    //             let first = e.components().next();
    //             assert!(first.is_some());
    //             match first.unwrap() {
    //                 Component::Normal(s) => s == modulename,
    //                 _                    => false,
    //             }
    //         }))
    //     }

    //     let store = get_store();
    //     for path in pathes {
    //         assert!(store.create(PathBuf::from(path)).is_ok());
    //     }

    //     test(&store, "foo");
    //     test(&store, "bar");
    //     test(&store, "bla");
    //     test(&store, "boo");
    //     test(&store, "glu");
    // }

    #[test]
    fn test_store_move_moves_in_hm() {
        use storeid::StoreId;

        let store = get_store();

        for n in 1..100 {
            if n % 2 == 0 { // every second
                let id    = StoreId::new_baseless(PathBuf::from(format!("t-{}", n))).unwrap();
                let id_mv = StoreId::new_baseless(PathBuf::from(format!("t-{}", n - 1))).unwrap();

                {
                    assert!(store.entries.read().unwrap().get(&id).is_none());
                }

                {
                    assert!(store.create(id.clone()).is_ok());
                }

                {
                    let id_with_base = id.clone().with_base(store.path().clone());
                    assert!(store.entries.read().unwrap().get(&id_with_base).is_some());
                }

                let r = store.move_by_id(id.clone(), id_mv.clone());
                assert!(r.map_err(|e| println!("ERROR: {:?}", e)).is_ok());

                {
                    let id_mv_with_base = id_mv.clone().with_base(store.path().clone());
                    assert!(store.entries.read().unwrap().get(&id_mv_with_base).is_some());
                }

                assert!(match store.get(id.clone()) { Ok(None) => true, _ => false },
                        "Moved id ({:?}) is still there", id);
                assert!(match store.get(id_mv.clone()) { Ok(Some(_)) => true, _ => false },
                        "New id ({:?}) is not in store...", id_mv);
            }
        }
    }

}

