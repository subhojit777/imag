use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::sync::Arc;
use std::sync::RWLock;

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
    cache: Arc<RWLock<HashMap<PathBuf, File>>>,
}

impl Store {

    pub fn create(entry: Entry) -> Result<()> {
        unimplemented!()
    }

    pub fn read(path: PathBuf) -> Result<Arc<RWLock<Entry>>> {
        unimplemented!()
    }

    pub fn update(entry: Arc<RWLock<Entry>>) -> Result<()> {
        unimplemented!()
    }

    pub fn delete(path: PathBuf) -> Result<()> {
        unimplemented!()
    }

}

