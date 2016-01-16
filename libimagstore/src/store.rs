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

pub trait Store : Sized {

    fn location(&self) -> &PathBuf;

    fn create(&self, entry: Entry) -> Result<()>;
    fn retrieve<'a>(&'a self, path: PathBuf) -> Result<FileLockEntry<'a, Self>>;
    fn update<'a>(&'a self, entry: FileLockEntry<'a, Self>) -> Result<()>;
    fn retrieve_copy(&self, id : String) -> Result<Entry>;
    fn delete(&self, path: PathBuf) -> Result<()>;

}

pub struct FileLockEntry<'a, S: Store + 'a> {
    store: &'a S,
    entry: Entry
}

impl<'a, S: Store + 'a> FileLockEntry<'a, S > {
    fn new(store: &'a S, entry: Entry) -> FileLockEntry<'a, S> {
        FileLockEntry {
            store: store,
            entry: entry,
        }
    }
}

impl<'a, S: Store + 'a> ::std::ops::Deref for FileLockEntry<'a, S> {
    type Target = Entry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl<'a, S: Store + 'a> ::std::ops::DerefMut for FileLockEntry<'a, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry
    }
}

