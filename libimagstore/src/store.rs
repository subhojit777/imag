use std::collections::HashMap;
use std::fs::File;
use std::ops::Drop;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::sync::Arc;
use std::sync::{RwLock, Mutex};

use fs2::FileExt;

use entry::Entry;
use error::StoreError;

pub type Result<T> = RResult<T, StoreError>;

pub type StoreId = PathBuf;

trait IntoStoreId {
    fn into_storeid(self) -> StoreId;
}

impl<'a> IntoStoreId for &'a str {
    fn into_storeid(self) -> StoreId {
        PathBuf::from(self)
    }
}

impl<'a> IntoStoreId for &'a String{
    fn into_storeid(self) -> StoreId {
        PathBuf::from(self)
    }
}

impl IntoStoreId for String{
    fn into_storeid(self) -> StoreId {
        PathBuf::from(self)
    }
}

impl IntoStoreId for PathBuf {
    fn into_storeid(self) -> StoreId {
        self
    }
}

impl<'a> IntoStoreId for &'a PathBuf {
    fn into_storeid(self) -> StoreId {
        self.clone()
    }
}

impl<ISI: IntoStoreId> IntoStoreId for (ISI, ISI) {
    fn into_storeid(self) -> StoreId {
        let (first, second) = self;
        let mut res : StoreId = first.into_storeid();
        res.push(second.into_storeid());
        res
    }
}


pub struct Store {
    location: PathBuf,

    /**
     * Internal Path->File cache map
     *
     * Caches the files, so they remain flock()ed
     *
     * Could be optimized for a threadsafe HashMap
     */
    entries: Arc<RwLock<HashMap<StoreId, (File, Option<Entry>)>>>,
}

impl Store {
    fn create(&self, entry: Entry) -> Result<()> {
        unimplemented!();
    }

    fn retrieve<'a>(&'a self, id: StoreId) -> Result<FileLockEntry<'a>> {
        unimplemented!();
    }

    fn update<'a>(&'a self, entry: FileLockEntry<'a>) -> Result<()> {
        unimplemented!();
    }

    fn retrieve_copy(&self, id: StoreId) -> Result<Entry> {
        unimplemented!();
    }

    fn delete(&self, id: StoreId) -> Result<()> {
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
        self.entries.write().unwrap()
            .iter().map(|f| (f.1).0.unlock());
    }

}

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
        let mut map = self.store.entries.write().unwrap();
        let (_, ref mut en) = *map.get_mut(&self.key).unwrap();
        *en = Some(self.entry.clone());
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use store::{StoreId, IntoStoreId};

    #[test]
    fn into_storeid_trait() {
        let buf = PathBuf::from("abc/def");

        let test = ("abc", "def");
        assert_eq!(buf, test.into_storeid());

        let test = "abc/def";
        assert_eq!(buf, test.into_storeid());

        let test = String::from("abc/def");
        assert_eq!(buf, test.into_storeid());

        let test = PathBuf::from("abc/def");
        assert_eq!(buf, test.into_storeid());
    }
}

