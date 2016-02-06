use std::collections::HashMap;
use std::fs::{File, remove_file};
use std::ops::Drop;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::sync::Arc;
use std::sync::RwLock;
use std::collections::BTreeMap;
use std::io::{Seek, SeekFrom};

use toml::{Table, Value};
use regex::Regex;

use error::{ParserErrorKind, ParserError};
use error::{StoreError, StoreErrorKind};
use storeid::{StoreId, StoreIdIterator};
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

    fn new(id: StoreId) -> StoreEntry {
        StoreEntry {
            id: id.clone(),
            file: LazyFile::Absent(id),
            status: StoreEntryStatus::Present,
        }
    }

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
                let mut file = file.unwrap();
                let entry = Entry::from_file(self.id.clone(), &mut file);
                file.seek(SeekFrom::Start(0));
                entry
            }
        } else {
            return Err(StoreError::new(StoreErrorKind::EntryAlreadyBorrowed, None))
        }
    }

    fn write_entry(&mut self, entry: &Entry) -> Result<()> {
        if self.is_borrowed() {
            use std::io::Write;
            let file = try!(self.file.create_file());

            assert_eq!(self.id, entry.location);
            file.write_all(entry.to_str().as_bytes());
        }

        Ok(())
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

        debug!("Building new Store object");
        if !location.exists() {
            debug!("Creating store path");
            let c = create_dir_all(location.clone());
            if c.is_err() {
                debug!("Failed");
                return Err(StoreError::new(StoreErrorKind::StorePathCreate,
                                           Some(Box::new(c.err().unwrap()))));
            }
        } else {
            if location.is_file() {
                debug!("Store path exists as file");
                return Err(StoreError::new(StoreErrorKind::StorePathExists, None));
            }
        }

        debug!("Store building succeeded");
        Ok(Store {
            location: location,
            entries: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Creates the Entry at the given location (inside the entry)
    pub fn create<'a>(&'a self, id: StoreId) -> Result<FileLockEntry<'a>> {
        let hsmap = self.entries.write();
        if hsmap.is_err() {
            return Err(StoreError::new(StoreErrorKind::LockPoisoned, None))
        }
        let mut hsmap = hsmap.unwrap();
        if hsmap.contains_key(&id) {
            return Err(StoreError::new(StoreErrorKind::EntryAlreadyExists, None))
        }
        hsmap.insert(id.clone(), {
            let mut se = StoreEntry::new(id.clone());
            se.status = StoreEntryStatus::Borrowed;
            se
        });
        Ok(FileLockEntry::new(self, Entry::new(id.clone()), id))
    }

    /// Borrow a given Entry. When the `FileLockEntry` is either `update`d or
    /// dropped, the new Entry is written to disk
    pub fn retrieve<'a>(&'a self, id: StoreId) -> Result<FileLockEntry<'a>> {
        self.entries
            .write()
            .map_err(|_| StoreError::new(StoreErrorKind::LockPoisoned, None))
            .and_then(|mut es| {
                let mut se = es.entry(id.clone()).or_insert_with(|| StoreEntry::new(id.clone()));
                let entry = se.get_entry();
                se.status = StoreEntryStatus::Borrowed;
                entry
            })
            .map(|e| FileLockEntry::new(self, e, id))
   }

    /// Iterate over all StoreIds for one module name
    pub fn retrieve_for_module(&self, mod_name: &str) -> StoreIdIterator {
        unimplemented!();
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
        let hsmap = self.entries.write();
        if hsmap.is_err() {
            return Err(StoreError::new(StoreErrorKind::LockPoisoned, None))
        }
        let mut hsmap = hsmap.unwrap();
        let mut se = try!(hsmap.get_mut(&entry.key)
              .ok_or(StoreError::new(StoreErrorKind::IdNotFound, None)));

        assert!(se.is_borrowed(), "Tried to update a non borrowed entry.");

        try!(se.write_entry(&entry.entry));
        se.status = StoreEntryStatus::Present;

        Ok(())
    }

    /// Retrieve a copy of a given entry, this cannot be used to mutate
    /// the one on disk
    pub fn retrieve_copy(&self, id: StoreId) -> Result<Entry> {
        unimplemented!();
    }

    /// Delete an entry
    pub fn delete(&self, id: StoreId) -> Result<()> {
        let mut entries_lock = self.entries.write();
        if entries_lock.is_err() {
            return Err(StoreError::new(StoreErrorKind::LockPoisoned, None))
        }

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
        debug!("Dropping store");
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

    pub fn toml_mut(&mut self) -> &mut Table {
        &mut self.toml
    }

    pub fn parse(s: &str) -> EntryResult<EntryHeader> {
        use toml::Parser;

        let mut parser = Parser::new(s);
        parser.parse()
            .ok_or(ParserError::new(ParserErrorKind::TOMLParserErrors, None))
            .and_then(verify_header_consistency)
            .map(EntryHeader::from_table)
    }

}

fn build_default_header() -> BTreeMap<String, Value> {
    let mut m = BTreeMap::new();

    m.insert(String::from("imag"), {
        let mut imag_map = BTreeMap::<String, Value>::new();

        imag_map.insert(String::from("version"), Value::String(String::from(version!())));
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

    pub fn new(loc: StoreId) -> Entry {
        Entry {
            location: loc,
            header: EntryHeader::new(),
            content: EntryContent::new()
        }
    }

    pub fn from_file(loc: StoreId, file: &mut File) -> Result<Entry> {
        let text = {
            use std::io::Read;
            let mut s = String::new();
            try!(file.read_to_string(&mut s));
            s
        };
        Self::from_str(loc, &text[..])
    }

    pub fn from_str(loc: StoreId, s: &str) -> Result<Entry> {
        debug!("Building entry from string");
        let re = Regex::new(r"(?smx)
            ^---$
            (?P<header>.*) # Header
            ^---$\n
            (?P<content>.*) # Content
        ").unwrap();

        let matches = re.captures(s);

        if matches.is_none() {
            return Err(StoreError::new(StoreErrorKind::MalformedEntry, None));
        }

        let matches = matches.unwrap();

        let header = matches.name("header");
        let content = matches.name("content").unwrap_or("");

        if header.is_none() {
            return Err(StoreError::new(StoreErrorKind::MalformedEntry, None));
        }

        debug!("Header and content found. Yay! Building Entry object now");
        Ok(Entry {
            location: loc,
            header: try!(EntryHeader::parse(header.unwrap())),
            content: content.into(),
        })
    }

    pub fn to_str(&self) -> String {
        format!("---{header}---\n{content}",
                header  = ::toml::encode_str(&self.header.toml),
                content = self.content)
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
        let entry = Entry::from_str(PathBuf::from("/test/foo~1.3"),
                                    TEST_ENTRY).unwrap();

        assert_eq!(entry.content, "Hai");
    }

    #[test]
    fn test_entry_to_str() {
        use super::Entry;
        use std::path::PathBuf;
        println!("{}", TEST_ENTRY);
        let entry = Entry::from_str(PathBuf::from("/test/foo~1.3"),
                                    TEST_ENTRY).unwrap();
        let string = entry.to_str();

        assert_eq!(TEST_ENTRY, string);
    }

}



