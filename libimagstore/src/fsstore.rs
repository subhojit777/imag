use std::collections::HashMap;
use std::fs::File;
use std::ops::Drop;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::sync::Arc;
use std::sync::RwLock;

pub use store::Store;
pub use store::Result;
pub use entry::Entry;
pub use error::StoreError;

pub struct FSStore {
    location: PathBuf,

    /**
     * Internal Path->File cache map
     *
     * Caches the files, so they remain flock()ed
     *
     * Could be optimized for a threadsafe HashMap
     */
    cache: Arc<RwLock<HashMap<PathBuf, File>>>,
}

impl FSStore {

    pub fn new(location: PathBuf) -> FSStore {
        FSStore {
            location: location,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Store for FSStore {

    fn location(&self) -> &PathBuf {
        &self.location
    }

    fn create(&self, entry: Entry) -> Result<()> {
        unimplemented!()
    }

    fn read(&self, path: PathBuf) -> Result<Arc<RwLock<Entry>>> {
        unimplemented!()
    }

    fn retrieve_copy(&self, id : String) -> Result<Entry> {
        unimplemented!()
    }

    fn update(&self, entry: Arc<RwLock<Entry>>) -> Result<()> {
        unimplemented!()
    }

    fn delete(&self, path: PathBuf) -> Result<()> {
        unimplemented!()
    }

}

impl Drop for FSStore {

    /**
     * Unlock all files on drop
     *
     * TODO: Error message when file cannot be unlocked?
     */
    fn drop(&mut self) {
        use std::ops::DerefMut;
        use fs2::FileExt;

        let cache = self.cache.clone();
        cache.write().unwrap().deref_mut().iter().map(|(_, f)| f.unlock());
    }

}


