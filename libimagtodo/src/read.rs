

use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::{FileLockEntry, Store};
use libimagstore::storeid::StoreId;
use module_path::ModuleEntryPath;
use error::{TodoError, TodoErrorKind};

use std::result::Result as RResult;

pub type Result<T> = RResult<T, TodoError>;

pub struct Read<'a> {
	 entry: FileLockEntry<'a>,
}

pub fn all_uuids(store: &Store) -> Result<ReadIterator> {
		store.retrieve_for_module("uuid")
			.map(|iter| ReadIterator::new(store, iter))
			.map_err(|e| TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e))))
}




trait FromStoreId {
    fn from_storeid<'a>(&'a Store, StoreId) -> Result<Read<'a>>;
}

impl<'a> FromStoreId for Read<'a> {

    fn from_storeid<'b>(store: &'b Store, id: StoreId) -> Result<Read<'b>> {
        match store.retrieve(id) {     
        	Err(e) => Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e)))),       
            Ok(c)  => Ok(Read { entry: c }),
        }
    }

}


pub struct ReadIterator<'a> {
    store: &'a Store,
    iditer: StoreIdIterator,
}

impl<'a> ReadIterator<'a> {

    pub fn new(store: &'a Store, iditer: StoreIdIterator) -> ReadIterator<'a> {
        ReadIterator {
            store: store,
            iditer: iditer,
        }
    }

}

impl<'a> Iterator for ReadIterator<'a> {
    type Item = Result<Read<'a>>;

    fn next(&mut self) -> Option<Result<Read<'a>>> {
        self.iditer
            .next()
            .map(|id| Read::from_storeid(self.store, id))
    }

}
