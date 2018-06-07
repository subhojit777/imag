//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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
use std::collections::BTreeMap;
use std::ops::Drop;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::sync::Arc;
use std::sync::RwLock;
use std::io::Read;
use std::ops::Deref;
use std::ops::DerefMut;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Error as FMTError;

use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::read::TomlValueReadTypeExt;

use error::{StoreError as SE, StoreErrorKind as SEK};
use error::ResultExt;
use storeid::{IntoStoreId, StoreId, StoreIdIteratorWithStore};
use file_abstraction::FileAbstractionInstance;

// We re-export the following things so tests can use them
pub use file_abstraction::FileAbstraction;
pub use file_abstraction::FSFileAbstraction;
pub use file_abstraction::InMemoryFileAbstraction;

use libimagutil::debug_result::*;

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

impl StoreEntry {

    fn new(id: StoreId, backend: &Arc<FileAbstraction>) -> Result<StoreEntry> {
        let pb = id.clone().into_pathbuf()?;

        #[cfg(feature = "fs-lock")]
        {
            open_file(pb.clone())
                .and_then(|f| f.lock_exclusive())
                .chain_err(|| SEK::IoError)?;
        }

        Ok(StoreEntry {
            id,
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
        if !self.is_borrowed() {
            self.file
                .get_file_content(self.id.clone())
                .or_else(|err| if is_match!(err.kind(), &SEK::FileNotFound) {
                    Ok(Entry::new(self.id.clone()))
                } else {
                    Err(err)
                })
        } else {
            Err(SE::from_kind(SEK::EntryAlreadyBorrowed(self.id.clone())))
        }
    }

    fn write_entry(&mut self, entry: &Entry) -> Result<()> {
        if self.is_borrowed() {
            assert_eq!(self.id, entry.location);
            trace!("Writing entry...");
            self.file
                .write_file_content(entry)
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
            .and_then(|entry| open_file(entry.get_location().clone()))
            .and_then(|f| f.unlock())
    }

}


/// The Store itself, through this object one can interact with IMAG's entries
pub struct Store {
    location: PathBuf,

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
    backend: Arc<FileAbstraction>,
}

impl Store {

    /// Create a new Store object
    ///
    /// This opens a Store in `location`. The store_config is used to check whether creating the
    /// store implicitely is allowed.
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
    ///
    pub fn new(location: PathBuf, store_config: &Option<Value>) -> Result<Store> {
        let backend = Arc::new(FSFileAbstraction::default());
        Store::new_with_backend(location, store_config, backend)
    }

    /// Create a Store object as descripbed in `Store::new()` documentation, but with an alternative
    /// backend implementation.
    ///
    /// Do not use directly, only for testing purposes.
    pub fn new_with_backend(location: PathBuf,
                            store_config: &Option<Value>,
                            backend: Arc<FileAbstraction>) -> Result<Store> {
        use configuration::*;

        debug!("Building new Store object");
        if !location.exists() {
            if !config_implicit_store_create_allowed(store_config)? {
                return Err(SE::from_kind(SEK::CreateStoreDirDenied))
                    .chain_err(|| SEK::FileError)
                    .chain_err(|| SEK::IoError);
            }

            backend
                .create_dir_all(&location)
                .chain_err(|| SEK::StorePathCreate(location.clone()))
                .map_dbg_err_str("Failed")?;
        } else if location.is_file() {
            debug!("Store path exists as file");
            return Err(SE::from_kind(SEK::StorePathExists(location)));
        }

        let store = Store {
            location: location.clone(),
            entries: Arc::new(RwLock::new(HashMap::new())),
            backend: backend,
        };

        debug!("Store building succeeded");
        debug!("------------------------");
        debug!("{:?}", store);
        debug!("------------------------");

        Ok(store)
    }

    /// Creates the Entry at the given location (inside the entry)
    ///
    /// # Return value
    ///
    /// On success: FileLockEntry
    ///
    /// On error:
    ///  - Errors StoreId::into_storeid() might return
    ///  - EntryAlreadyExists(id) if the entry exists
    ///  - CreateCallError(LockPoisoned()) if the internal lock is poisened.
    ///  - CreateCallError(EntryAlreadyExists()) if the entry exists already.
    ///
    pub fn create<'a, S: IntoStoreId>(&'a self, id: S) -> Result<FileLockEntry<'a>> {
        let id = id.into_storeid()?.with_base(self.path().clone());

        debug!("Creating id: '{}'", id);

        let exists = id.exists()? || self.entries
            .read()
            .map(|map| map.contains_key(&id))
            .map_err(|_| SE::from_kind(SEK::LockPoisoned))
            .chain_err(|| SEK::CreateCallError(id.clone()))?;

        if exists {
            debug!("Entry exists: {:?}", id);
            return Err(SEK::EntryAlreadyExists(id).into());
        }

        {
            let mut hsmap = self
                .entries
                .write()
                .map_err(|_| SE::from_kind(SEK::LockPoisoned))
                .chain_err(|| SEK::CreateCallError(id.clone()))?;

            if hsmap.contains_key(&id) {
                debug!("Cannot create, internal cache already contains: '{}'", id);
                return Err(SE::from_kind(SEK::EntryAlreadyExists(id.clone())))
                           .chain_err(|| SEK::CreateCallError(id.clone()));
            }
            hsmap.insert(id.clone(), {
                debug!("Creating: '{}'", id);
                let mut se = StoreEntry::new(id.clone(), &self.backend)?;
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
        let id = id.into_storeid()?.with_base(self.path().clone());
        debug!("Retrieving id: '{}'", id);
        let entry = self
            .entries
            .write()
            .map_err(|_| SE::from_kind(SEK::LockPoisoned))
            .and_then(|mut es| {
                let new_se = StoreEntry::new(id.clone(), &self.backend)?;
                let se = es.entry(id.clone()).or_insert(new_se);
                let entry = se.get_entry();
                se.status = StoreEntryStatus::Borrowed;
                entry
            })
            .chain_err(|| SEK::RetrieveCallError(id.clone()))?;

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
        let id = id.into_storeid()?.with_base(self.path().clone());

        debug!("Getting id: '{}'", id);

        let exists = id.exists()? || self.entries
            .read()
            .map(|map| map.contains_key(&id))
            .map_err(|_| SE::from_kind(SEK::LockPoisoned))
            .chain_err(|| SEK::GetCallError(id.clone()))?;

        if !exists {
            debug!("Does not exist in internal cache or filesystem: {:?}", id);
            return Ok(None);
        }

        self.retrieve(id.clone()).map(Some).chain_err(|| SEK::GetCallError(id))
    }

    /// Write (update) the `FileLockEntry` to disk
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
    pub fn update<'a>(&'a self, entry: &mut FileLockEntry<'a>) -> Result<()> {
        debug!("Updating FileLockEntry at '{}'", entry.get_location());
        self._update(entry, false).chain_err(|| SEK::UpdateCallError(entry.get_location().clone()))
    }

    /// Internal method to write to the filesystem store.
    ///
    /// # Assumptions
    ///
    /// This method assumes that entry is dropped _right after_ the call, hence
    /// it is not public.
    ///
    fn _update<'a>(&'a self, entry: &mut FileLockEntry<'a>, modify_presence: bool) -> Result<()> {
        let mut hsmap = self.entries.write().map_err(|_| SE::from_kind(SEK::LockPoisoned))?;

        let se = hsmap.get_mut(&entry.location).ok_or_else(|| {
            SE::from_kind(SEK::IdNotFound(entry.location.clone()))
        })?;

        assert!(se.is_borrowed(), "Tried to update a non borrowed entry.");

        debug!("Verifying Entry");
        entry.entry.verify()?;

        debug!("Writing Entry");
        se.write_entry(&entry.entry)?;
        trace!("Entry written");
        if modify_presence {
            debug!("Modifying presence of {} -> Present", entry.get_location());
            se.status = StoreEntryStatus::Present;
        }

        trace!("Entry updated successfully");
        Ok(())
    }

    /// Flush the store internal cache
    ///
    /// This is helpful if a lot of entries are beeing read/written, because the store holds the
    /// file handles internally. At some point, the OS simply errors with "Too many files open".
    /// With this function, not-borrowed entries can be flushed back to disk and thus file handles
    /// are dropped.
    ///
    /// After the flushables are dropped, the internal cache is shrinked to fit the number of
    /// elements still in the cache.
    ///
    pub fn flush_cache(&self) -> Result<()> {
        // We borrow this early so that between the aggregation of the flushables and the actual
        // flush, there is no borrowing from the store.
        let mut hsmap = self.entries.write().map_err(|_| SE::from_kind(SEK::LockPoisoned))?;
        let mut to_flush = vec![];

        for (storeid, se) in hsmap.deref() {
            if !se.is_borrowed() {
                to_flush.push(storeid.clone());
            }
        }

        for id in to_flush {
            let _ = hsmap.remove(&id);
        }

        hsmap.shrink_to_fit();

        Ok(())
    }

    /// The number of elements in the internal cache
    pub fn cache_size(&self) -> Result<usize> {
        let hsmap = self.entries.read().map_err(|_| SE::from_kind(SEK::LockPoisoned))?;
        Ok(hsmap.iter().count())
    }

    /// The size of the internal cache
    pub fn cache_capacity(&self) -> Result<usize> {
        let hsmap = self.entries.read().map_err(|_| SE::from_kind(SEK::LockPoisoned))?;
        Ok(hsmap.capacity())
    }

    /// Get a copy of a given entry, this cannot be used to mutate the one on disk
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
    pub fn get_copy<S: IntoStoreId>(&self, id: S) -> Result<Entry> {
        let id = id.into_storeid()?.with_base(self.path().clone());
        debug!("Retrieving copy of '{}'", id);
        let entries = self.entries.write()
            .map_err(|_| SE::from_kind(SEK::LockPoisoned))
            .chain_err(|| SEK::RetrieveCopyCallError(id.clone()))?;

        // if the entry is currently modified by the user, we cannot drop it
        if entries.get(&id).map(|e| e.is_borrowed()).unwrap_or(false) {
            return Err(SE::from_kind(SEK::IdLocked)).chain_err(|| SEK::RetrieveCopyCallError(id));
        }

        StoreEntry::new(id, &self.backend)?.get_entry()
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
        let id = id.into_storeid()?.with_base(self.path().clone());

        debug!("Deleting id: '{}'", id);

        {
            let mut entries = self
                .entries
                .write()
                .map_err(|_| SE::from_kind(SEK::LockPoisoned))
                .chain_err(|| SEK::DeleteCallError(id.clone()))?;

            // if the entry is currently modified by the user, we cannot drop it
            match entries.get(&id) {
                None => {
                    // The entry is not in the internal cache. But maybe on the filesystem?
                    debug!("Seems like {:?} is not in the internal cache", id);

                    // Small optimization: We need the pathbuf for deleting, but when calling
                    // StoreId::exists(), a PathBuf object gets allocated. So we simply get a
                    // PathBuf here, check whether it is there and if it is, we can re-use it to
                    // delete the filesystem file.
                    let pb = id.clone().into_pathbuf()?;

                    if self.backend.exists(&pb)? {
                        // looks like we're deleting a not-loaded file from the store.
                        debug!("Seems like {:?} is on the FS", pb);
                        return self.backend.remove_file(&pb)
                    } else {
                        debug!("Seems like {:?} is not even on the FS", pb);
                        return Err(SE::from_kind(SEK::FileNotFound))
                            .chain_err(|| SEK::DeleteCallError(id))
                    }
                },
                Some(e) => if e.is_borrowed() {
                    return Err(SE::from_kind(SEK::IdLocked))
                        .chain_err(|| SEK::DeleteCallError(id))
                }
            }

            // remove the entry first, then the file
            entries.remove(&id);
            let pb = id.clone().with_base(self.path().clone()).into_pathbuf()?;
            let _ = self
                .backend
                .remove_file(&pb)
                .chain_err(|| SEK::FileError)
                .chain_err(|| SEK::DeleteCallError(id))?;
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
        let hsmap = self
            .entries
            .write()
            .map_err(|_| SE::from_kind(SEK::LockPoisoned))
            .chain_err(|| SEK::MoveCallError(entry.get_location().clone(), new_id.clone()))?;

        if hsmap.contains_key(&new_id) {
            return Err(SE::from_kind(SEK::EntryAlreadyExists(new_id.clone())))
                .chain_err(|| SEK::MoveCallError(entry.get_location().clone(), new_id.clone()))
        }

        let old_id = entry.get_location().clone();

        let old_id_as_path = old_id.clone().with_base(self.path().clone()).into_pathbuf()?;
        let new_id_as_path = new_id.clone().with_base(self.path().clone()).into_pathbuf()?;
        self.backend
            .copy(&old_id_as_path, &new_id_as_path)
            .and_then(|_| if remove_old {
                debug!("Removing old '{:?}'", old_id_as_path);
                self.backend.remove_file(&old_id_as_path)
            } else {
                Ok(())
            })
            .chain_err(|| SEK::FileError)
            .chain_err(|| SEK::MoveCallError(old_id, new_id))
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
            let mut hsmap = self.entries.write().map_err(|_| SE::from_kind(SEK::LockPoisoned))?;

            if hsmap.contains_key(&new_id) {
                return Err(SE::from_kind(SEK::EntryAlreadyExists(new_id.clone())));
            }
            debug!("New id does not exist in cache");

            // if we do not have an entry here, we fail in `FileAbstraction::rename()` below.
            // if we have one, but it is borrowed, we really should not rename it, as this might
            // lead to strange errors
            if hsmap.get(&old_id).map(|e| e.is_borrowed()).unwrap_or(false) {
                return Err(SE::from_kind(SEK::EntryAlreadyBorrowed(old_id.clone())));
            }

            debug!("Old id is not yet borrowed");

            let old_id_pb = old_id.clone().with_base(self.path().clone()).into_pathbuf()?;
            let new_id_pb = new_id.clone().with_base(self.path().clone()).into_pathbuf()?;

            if self.backend.exists(&new_id_pb)? {
                return Err(SE::from_kind(SEK::EntryAlreadyExists(new_id.clone())));
            }
            debug!("New entry does not yet exist on filesystem. Good.");

            let _ = self
                .backend
                .rename(&old_id_pb, &new_id_pb)
                .chain_err(|| SEK::EntryRenameError(old_id_pb, new_id_pb))?;

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

        debug!("Moved");
        Ok(())
    }

    /// Get _all_ entries in the store (by id as iterator)
    pub fn entries(&self) -> Result<StoreIdIteratorWithStore> {
        self.backend
            .pathes_recursively(self.path().clone())
            .map(|i| i.store_id_constructing(self.path().clone(), self.backend.clone()))
            .map(Box::new)
            .map(|it| StoreIdIteratorWithStore::new(it, self))
    }

    /// Gets the path where this store is on the disk
    pub fn path(&self) -> &PathBuf {
        &self.location
    }

}

impl Debug for Store {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FMTError> {
        writeln!(fmt, "Store location = {:?}, entries = {:?}", self.location, self.entries)
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
        FileLockEntry { store, entry }
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
        trace!("Dropping: {:?} - from FileLockEntry::drop()", self.get_location());
        if let Err(e) = self.store._update(self, true) {
            trace!("Error happened in FileLockEntry::drop() while Store::update()ing");
            trace_error_dbg(&e);
            if_cfg_panic!("ERROR WHILE DROPPING: {:?}", e);
        }
    }
}

#[cfg(test)]
impl<'a> Drop for FileLockEntry<'a> {

    /// This will not silently ignore errors but prints the result of the _update() call for testing
    fn drop(&mut self) {
        use libimagerror::trace::trace_error;

        trace!("Dropping: {:?} - from FileLockEntry::drop() (test impl)", self.get_location());
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
            file.read_to_string(&mut s)?;
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
        use util::entry_buffer_to_header_content;

        let (header, content) = entry_buffer_to_header_content(s)?;

        Ok(Entry {
            location: loc.into_storeid()?,
            header,
            content,
        })
    }

    /// Return the string representation of this entry
    ///
    /// This means not only the content of the entry, but the complete entry (from memory, not from
    /// disk).
    pub fn to_str(&self) -> Result<String> {
        Ok(format!("---\n{header}---\n{content}",
                   header  = ::toml::ser::to_string_pretty(&self.header)?,
                   content = self.content))
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

    /// Replace both header and content of the entry by reading from buffer
    ///
    /// If an error is returned, the contents of neither the header nor the content are modified.
    pub fn replace_from_buffer(&mut self, buf: &str) -> Result<()> {
        let (header, content) = ::util::entry_buffer_to_header_content(buf)?;
        self.content          = content;
        self.header           = header;
        Ok(())
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

/// Extension trait for top-level toml::Value::Table, will only yield correct results on the
/// top-level Value::Table, but not on intermediate tables.
pub trait Header {
    fn verify(&self) -> Result<()>;
    fn parse(s: &str) -> Result<Value>;
    fn default_header() -> Value;
}

impl Header for Value {

    fn verify(&self) -> Result<()> {
        if !has_main_section(self)? {
            Err(SE::from_kind(SEK::MissingMainSection))
        } else if !has_imag_version_in_main_section(self)? {
            Err(SE::from_kind(SEK::MissingVersionInfo))
        } else if !has_only_tables(self)? {
            debug!("Could not verify that it only has tables in its base table");
            Err(SE::from_kind(SEK::NonTableInBaseTable))
        } else {
            Ok(())
        }
    }

    fn parse(s: &str) -> Result<Value> {
        use toml::de::from_str;

        from_str(s)
            .map_err(From::from)
            .and_then(|h: Value| h.verify().map(|_| h))
    }

    fn default_header() -> Value {
        let mut m = BTreeMap::new();

        m.insert(String::from("imag"), {
            let mut imag_map = BTreeMap::<String, Value>::new();

            imag_map.insert(String::from("version"),
                Value::String(String::from(env!("CARGO_PKG_VERSION"))));

            Value::Table(imag_map)
        });

        Value::Table(m)
    }

}

fn has_only_tables(t: &Value) -> Result<bool> {
    debug!("Verifying that table has only tables");
    match *t {
        Value::Table(ref tab) => Ok(tab.iter().all(|(_, x)| is_match!(*x, Value::Table(_)))),
        _ => Err(SE::from_kind(SEK::HeaderTypeFailure)),
    }
}

fn has_main_section(t: &Value) -> Result<bool> {
    t.read("imag")?
        .ok_or_else(|| SE::from_kind(SEK::ConfigKeyMissingError("imag")))
        .map(Value::is_table)
}

fn has_imag_version_in_main_section(t: &Value) -> Result<bool> {
    t.read_string("imag.version")?
        .ok_or_else(|| SE::from_kind(SEK::ConfigKeyMissingError("imag.version")))
        .map(String::from)
        .map(|s| ::semver::Version::parse(&s).is_ok())
}


#[cfg(test)]
mod test {
    extern crate env_logger;

    use std::collections::BTreeMap;
    use storeid::StoreId;
    use store::Header;
    use store::has_main_section;
    use store::has_imag_version_in_main_section;

    use toml::Value;

    #[test]
    fn test_imag_section() {
        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Table(BTreeMap::new()));

        assert!(has_main_section(&Value::Table(map)).unwrap());
    }

    #[test]
    fn test_imag_abscent_main_section() {
        let mut map = BTreeMap::new();
        map.insert("not_imag".into(), Value::Boolean(false));

        assert!(has_main_section(&Value::Table(map)).is_err());
    }

    #[test]
    fn test_main_section_without_version() {
        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Table(BTreeMap::new()));

        assert!(has_imag_version_in_main_section(&Value::Table(map)).is_err());
    }

    #[test]
    fn test_main_section_with_version() {
        let mut map = BTreeMap::new();
        let mut sub = BTreeMap::new();
        sub.insert("version".into(), Value::String("0.0.0".into()));
        map.insert("imag".into(), Value::Table(sub));

        assert!(has_imag_version_in_main_section(&Value::Table(map)).unwrap());
    }

    #[test]
    fn test_main_section_with_version_in_wrong_type() {
        let mut map = BTreeMap::new();
        let mut sub = BTreeMap::new();
        sub.insert("version".into(), Value::Boolean(false));
        map.insert("imag".into(), Value::Table(sub));

        assert!(has_imag_version_in_main_section(&Value::Table(map)).is_err());
    }

    #[test]
    fn test_verification_good() {
        let mut header = BTreeMap::new();
        let sub = {
            let mut sub = BTreeMap::new();
            sub.insert("version".into(), Value::String(String::from("0.0.0")));

            Value::Table(sub)
        };

        header.insert("imag".into(), sub);

        assert!(Value::Table(header).verify().is_ok());
    }

    #[test]
    fn test_verification_invalid_versionstring() {
        let mut header = BTreeMap::new();
        let sub = {
            let mut sub = BTreeMap::new();
            sub.insert("version".into(), Value::String(String::from("000")));

            Value::Table(sub)
        };

        header.insert("imag".into(), sub);

        assert!(!Value::Table(header).verify().is_ok());
    }


    #[test]
    fn test_verification_current_version() {
        let mut header = BTreeMap::new();
        let sub = {
            let mut sub = BTreeMap::new();
            sub.insert("version".into(), Value::String(String::from(env!("CARGO_PKG_VERSION"))));

            Value::Table(sub)
        };

        header.insert("imag".into(), sub);

        assert!(Value::Table(header).verify().is_ok());
    }

    static TEST_ENTRY : &'static str = "---
[imag]
version = '0.0.3'
---
Hai";

    static TEST_ENTRY_TNL : &'static str = "---
[imag]
version = '0.0.3'
---
Hai

";

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
        let string = entry.to_str().unwrap();

        assert_eq!(TEST_ENTRY, string);
    }

    #[test]
    fn test_entry_to_str_trailing_newline() {
        use super::Entry;
        use std::path::PathBuf;
        println!("{}", TEST_ENTRY_TNL);
        let entry = Entry::from_str(StoreId::new_baseless(PathBuf::from("test/foo~1.3")).unwrap(),
                                    TEST_ENTRY_TNL).unwrap();
        let string = entry.to_str().unwrap();

        assert_eq!(TEST_ENTRY_TNL, string);
    }
}

#[cfg(test)]
mod store_tests {
    use std::path::PathBuf;
    use std::sync::Arc;

    use super::Store;
    use file_abstraction::InMemoryFileAbstraction;

    pub fn get_store() -> Store {
        let backend = Arc::new(InMemoryFileAbstraction::default());
        Store::new_with_backend(PathBuf::from("/"), &None, backend).unwrap()
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
                .map_err(|e| assert!(is_match!(e.kind(), &SEK::EntryAlreadyExists(_)) && n >= 50))
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

