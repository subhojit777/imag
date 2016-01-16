use std::collections::HashMap;
use std::fs::File;
use std::ops::Drop;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::sync::Arc;
use std::sync::RwLock;

pub use entry::Entry;
pub use error::StoreError;

pub type Result<T> = RResult<T, StoreError>;

pub struct Store {
    location: PathBuf,

    /**
     * Internal Path->File cache map
     *
     * Caches the files, so they remain flock()ed
     *
     * Could be optimized for a threadsafe HashMap
     */
    cache: Arc<RwLock<HashMap<PathBuf, RwLock<File>>>>,
}

impl Store {

    fn create(&self, entry: Entry) -> Result<()> {
        unimplemented!();
    }
    fn retrieve<'a>(&'a self, path: PathBuf) -> Result<FileLockEntry<'a>> {
        unimplemented!();
    }
    fn update<'a>(&'a self, entry: FileLockEntry<'a>) -> Result<()> {
        unimplemented!();
    }
    fn retrieve_copy(&self, id : String) -> Result<Entry> {
        unimplemented!();
    }
    fn delete(&self, path: PathBuf) -> Result<()> {
        unimplemented!();
    }
}

impl Drop for Store {

    /**
     * Unlock all files on drop
     *
     * TODO: Error message when file cannot be unlocked?
     */
    fn drop(&mut self) {
        self.cache.iter().map(|f| f.unlock());
    }

}

pub struct FileLockEntry<'a> {
    store: &'a Store,
    entry: Entry
}

impl<'a> FileLockEntry<'a, > {
    fn new(store: &'a Store, entry: Entry) -> FileLockEntry<'a> {
        FileLockEntry {
            store: store,
            entry: entry,
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

