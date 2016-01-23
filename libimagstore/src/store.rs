use std::collections::HashMap;
use std::fs::{File, remove_file};
use std::ops::Drop;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::sync::Arc;
use std::sync::RwLock;
use std::error::Error;
use std::collections::BTreeMap;


use fs2::FileExt;
use toml::{Table, Value};
use regex::Regex;

use error::{ParserErrorKind, ParserError};
use error::{StoreError, StoreErrorKind};
use storeid::StoreId;
use lazyfile::LazyFile;

/// The Result Type returned by any interaction with the store that could fail
pub type Result<T> = RResult<T, StoreError>;


#[derive(PartialEq)]
enum StoreEntryStatus {
    Present,
    Borrowed
}

/// A store entry, depending on the option type it is either borrowed currently
/// or not.
struct StoreEntry {
    id: StoreId,
    file: LazyFile,
    status: StoreEntryStatus,
}

impl StoreEntry {
    /// The entry is currently borrowed, meaning that some thread is currently
    /// mutating it
    fn is_borrowed(&self) -> bool {
        self.status == StoreEntryStatus::Borrowed
    }

    fn get_entry(&mut self) -> Result<Entry> {
        if !self.is_borrowed() {
            let file = self.file.get_file_mut();
            if let Err(err) = file {
                if err.err_type() == StoreErrorKind::FileNotFound {
                    Ok(Entry::new(self.id.clone()))
                } else {
                    Err(err)
                }
            } else {
                // TODO:
                Entry::from_file(self.id.clone(), file.unwrap())
            }
        } else {
            return Err(StoreError::new(StoreErrorKind::EntryAlreadyBorrowed, None))
        }
    }
}

/// The Store itself, through this object one can interact with IMAG's entries
pub struct Store {
    location: PathBuf,

    /**
     * Internal Path->File cache map
     *
     * Caches the files, so they remain flock()ed
     *
     * Could be optimized for a threadsafe HashMap
     */
    entries: Arc<RwLock<HashMap<StoreId, StoreEntry>>>,
}

impl Store {

    /// Create a new Store object
    pub fn new(location: PathBuf) -> Result<Store> {
        use std::fs::create_dir_all;

        if !location.exists() {
            let c = create_dir_all(location.clone());
            if c.is_err() {
                return Err(StoreError::new(StoreErrorKind::StorePathCreate,
                                           Some(Box::new(c.err().unwrap()))));
            }
        } else {
            if location.is_file() {
                return Err(StoreError::new(StoreErrorKind::StorePathExists, None));
            }
        }

        Ok(Store {
            location: location,
            entries: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Creates the Entry at the given location (inside the entry)
    pub fn create(&self, entry: Entry) -> Result<()> {
        unimplemented!();
    }

    /// Borrow a given Entry. When the `FileLockEntry` is either `update`d or
    /// dropped, the new Entry is written to disk
    pub fn retrieve<'a>(&'a self, id: StoreId) -> Result<FileLockEntry<'a>> {
        let hsmap = self.entries.write();
        if hsmap.is_err() {
            return Err(StoreError::new(StoreErrorKind::LockPoisoned, None))
        }
        hsmap.unwrap().get_mut(&id)
            .ok_or(StoreError::new(StoreErrorKind::IdNotFound, None))
            .and_then(|store_entry| store_entry.get_entry())
            .and_then(|entry| Ok(FileLockEntry::new(self, entry, id)))
    }

    /// Return the `FileLockEntry` and write to disk
    pub fn update<'a>(&'a self, entry: FileLockEntry<'a>) -> Result<()> {
        self._update(&entry)
    }

    /// Internal method to write to the filesystem store.
    ///
    /// # Assumptions
    /// This method assumes that entry is dropped _right after_ the call, hence
    /// it is not public.
    fn _update<'a>(&'a self, entry: &FileLockEntry<'a>) -> Result<()> {
        unimplemented!();
    }

    /// Retrieve a copy of a given entry, this cannot be used to mutate
    /// the one on disk
    pub fn retrieve_copy(&self, id: StoreId) -> Result<Entry> {
        unimplemented!();
    }

    /// Delete an entry
    pub fn delete(&self, id: StoreId) -> Result<()> {
        let mut entries_lock = self.entries.write();
        let mut entries = entries_lock.unwrap();

        // if the entry is currently modified by the user, we cannot drop it
        if entries.get(&id).map(|e| e.is_borrowed()).unwrap_or(false) {
            return Err(StoreError::new(StoreErrorKind::IdLocked, None));
        }

        // remove the entry first, then the file
        entries.remove(&id);
        remove_file(&id).map_err(|e| StoreError::new(StoreErrorKind::FileError, Some(Box::new(e))))
    }
}

impl Drop for Store {

    /**
     * Unlock all files on drop
     *
     * TODO: Unlock them
     */
    fn drop(&mut self) {
    }

}

/// A struct that allows you to borrow an Entry
pub struct FileLockEntry<'a> {
    store: &'a Store,
    entry: Entry,
    key: StoreId,
}

impl<'a> FileLockEntry<'a, > {
    fn new(store: &'a Store, entry: Entry, key: StoreId) -> FileLockEntry<'a> {
        FileLockEntry {
            store: store,
            entry: entry,
            key: key,
        }
    }
}

impl<'a> ::std::ops::Deref for FileLockEntry<'a> {
    type Target = Entry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl<'a> ::std::ops::DerefMut for FileLockEntry<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry
    }
}

impl<'a> Drop for FileLockEntry<'a> {
    fn drop(&mut self) {
        self.store._update(self).unwrap()
    }
}

/**
 * EntryContent type
 */
pub type EntryContent = String;

/**
 * EntryHeader
 *
 * This is basically a wrapper around toml::Table which provides convenience to the user of the
 * librray.
 */
#[derive(Debug, Clone)]
pub struct EntryHeader {
    toml: Table,
}

pub type EntryResult<V> = RResult<V, ParserError>;

/**
 * Wrapper type around file header (TOML) object
 */
impl EntryHeader {

    pub fn new() -> EntryHeader {
        EntryHeader {
            toml: build_default_header()
        }
    }

    fn from_table(t: Table) -> EntryHeader {
        EntryHeader {
            toml: t
        }
    }

    /**
     * Get the table which lives in the background
     */
    pub fn toml(&self) -> &Table {
        &self.toml
    }

    pub fn parse(s: &str) -> EntryResult<EntryHeader> {
        use toml::Parser;

        let mut parser = Parser::new(s);
        parser.parse()
            .ok_or(ParserError::new(ParserErrorKind::TOMLParserErrors, None))
            .and_then(|t| verify_header_consistency(t))
            .map(|t| EntryHeader::from_table(t))
    }

}

fn build_default_header() -> BTreeMap<String, Value> {
    let mut m = BTreeMap::new();

    m.insert(String::from("imag"), {
        let mut imag_map = BTreeMap::<String, Value>::new();

        imag_map.insert(String::from("version"), Value::String(version!()));
        imag_map.insert(String::from("links"), Value::Array(vec![]));

        Value::Table(imag_map)
    });

    m
}

fn verify_header_consistency(t: Table) -> EntryResult<Table> {
    if !has_main_section(&t) {
        Err(ParserError::new(ParserErrorKind::MissingMainSection, None))
    } else if !has_imag_version_in_main_section(&t) {
        Err(ParserError::new(ParserErrorKind::MissingVersionInfo, None))
    } else {
        Ok(t)
    }
}

fn has_main_section(t: &Table) -> bool {
    t.contains_key("imag") &&
        match t.get("imag") {
            Some(&Value::Table(_)) => true,
            Some(_)                => false,
            None                   => false,
        }
}

fn has_imag_version_in_main_section(t: &Table) -> bool {
    use semver::Version;

    match t.get("imag").unwrap() {
        &Value::Table(ref sec) => {
            sec.get("version")
                .and_then(|v| {
                    match v {
                        &Value::String(ref s) => {
                            Some(Version::parse(&s[..]).is_ok())
                        },
                        _                 => Some(false),
                    }
                })
            .unwrap_or(false)
        }
        _                  => false,
    }
}

/**
 * An Entry of the store
 *
 * Contains location, header and content part.
 */
#[derive(Debug, Clone)]
pub struct Entry {
    location: StoreId,
    header: EntryHeader,
    content: EntryContent,
}

impl Entry {

    fn new(loc: StoreId) -> Entry {
        Entry {
            location: loc,
            header: EntryHeader::new(),
            content: EntryContent::new()
        }
    }

    fn from_file(loc: StoreId, file: &mut File) -> Result<Entry> {
        use std::io::Read;
        let file = {
            let mut buff = String::new();
            file.read_to_string(&mut buff);
            buff
        };

        let re = Regex::new(r"(?smx)
            ^---$
            (?P<header>.*) # Header
            ^---$
            (?P<content>.*) # Content
        ").unwrap();

        let matches = re.captures(&file[..]);

        if matches.is_none() {
            return Err(StoreError::new(StoreErrorKind::MalformedEntry, None));
        }

        let matches = matches.unwrap();

        let header = matches.name("header");
        let content = matches.name("content").unwrap_or("");

        if header.is_none() {
            return Err(StoreError::new(StoreErrorKind::MalformedEntry, None));
        }

        Ok(Entry {
            location: loc,
            header: try!(EntryHeader::parse(header.unwrap())),
            content: content.into(),
        })
    }

    pub fn get_location(&self) -> &StoreId {
        &self.location
    }

    pub fn get_header(&self) -> &EntryHeader {
        &self.header
    }

    pub fn get_header_mut(&mut self) -> &mut EntryHeader {
        &mut self.header
    }

    pub fn get_content(&self) -> &EntryContent {
        &self.content
    }

    pub fn get_content_mut(&mut self) -> &mut EntryContent {
        &mut self.content
    }

}


#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use toml::Value;

    #[test]
    fn test_imag_section() {
        use super::has_main_section;

        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Table(BTreeMap::new()));

        assert!(has_main_section(&map));
    }

    #[test]
    fn test_imag_invalid_section_type() {
        use super::has_main_section;

        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Boolean(false));

        assert!(!has_main_section(&map));
    }

    #[test]
    fn test_imag_abscent_main_section() {
        use super::has_main_section;

        let mut map = BTreeMap::new();
        map.insert("not_imag".into(), Value::Boolean(false));

        assert!(!has_main_section(&map));
    }

    #[test]
    fn test_main_section_without_version() {
        use super::has_imag_version_in_main_section;

        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Table(BTreeMap::new()));

        assert!(!has_imag_version_in_main_section(&map));
    }

    #[test]
    fn test_main_section_with_version() {
        use super::has_imag_version_in_main_section;

        let mut map = BTreeMap::new();
        let mut sub = BTreeMap::new();
        sub.insert("version".into(), Value::String("0.0.0".into()));
        map.insert("imag".into(), Value::Table(sub));

        assert!(has_imag_version_in_main_section(&map));
    }

    #[test]
    fn test_main_section_with_version_in_wrong_type() {
        use super::has_imag_version_in_main_section;

        let mut map = BTreeMap::new();
        let mut sub = BTreeMap::new();
        sub.insert("version".into(), Value::Boolean(false));
        map.insert("imag".into(), Value::Table(sub));

        assert!(!has_imag_version_in_main_section(&map));
    }

    #[test]
    fn test_verification_good() {
        use super::verify_header_consistency;

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
        use super::verify_header_consistency;

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
        use version;

        use super::verify_header_consistency;

        let mut header = BTreeMap::new();
        let sub = {
            let mut sub = BTreeMap::new();
            sub.insert("version".into(), Value::String(version!()));

            Value::Table(sub)
        };

        header.insert("imag".into(), sub);

        assert!(verify_header_consistency(header).is_ok());
    }
}



