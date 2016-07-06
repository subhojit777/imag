use libimagstore::storeid::{StoreIdIterator, StoreId};
use libimagstore::store::Store;
use error::{TodoError, TodoErrorKind};
use task::Task;
use result::Result;

pub fn get_todo_iterator(store: &Store) -> Result<TaskIterator> {
    store.retrieve_for_module("todo/taskwarrior")
        .map(|iter| TaskIterator::new(store, iter))
        .map_err(|e| TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e))))
}

trait FromStoreId {
    fn from_storeid<'a>(&'a Store, StoreId) -> Result<Task<'a>>;
}

impl<'a> FromStoreId for Task<'a> {

    fn from_storeid<'b>(store: &'b Store, id: StoreId) -> Result<Task<'b>> {
        match store.retrieve(id) {
            Err(e) => Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e)))),
            Ok(c)  => Ok(Task::new( c )),
        }
    }
}

pub struct TaskIterator<'a> {
    store: &'a Store,
    iditer: StoreIdIterator,
}

impl<'a> TaskIterator<'a> {

    pub fn new(store: &'a Store, iditer: StoreIdIterator) -> TaskIterator<'a> {
        TaskIterator {
            store: store,
            iditer: iditer,
        }
    }

}

impl<'a> Iterator for TaskIterator<'a> {
    type Item = Result<Task<'a>>;

    fn next(&mut self) -> Option<Result<Task<'a>>> {
        self.iditer.next().map(|id| Task::from_storeid(self.store, id))
    }
}
