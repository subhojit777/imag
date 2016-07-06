use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use toml::Value;

use task_hookrs::task::Task as TTask;

use libimagstore::store::{FileLockEntry, Store};
use libimagstore::storeid::IntoStoreId;
use module_path::ModuleEntryPath;

use error::{TodoError, TodoErrorKind};
use result::Result;

/// Task struct containing a `FileLockEntry`
#[derive(Debug)]
pub struct Task<'a>(FileLockEntry<'a>);

impl<'a> Task<'a> {

    /// Concstructs a new `Task` with a `FileLockEntry`
    pub fn new(fle: FileLockEntry<'a>) -> Task<'a> {
        Task(fle)
    }

}

impl<'a> Deref for Task<'a> {
    type Target = FileLockEntry<'a>;

    fn deref(&self) -> &FileLockEntry<'a> {
        &self.0
    }

}

impl<'a> DerefMut for Task<'a> {

    fn deref_mut(&mut self) -> &mut FileLockEntry<'a> {
        &mut self.0
    }

}

/// A trait to get a `libimagtodo::task::Task` out of the implementing object.
/// This Task struct is merely a wrapper for a `FileLockEntry`, therefore the function name
/// `into_filelockentry`.
pub trait IntoTask<'a> {

    /// # Usage
    /// ```ignore
    /// use std::io::stdin;
    ///
    /// use task_hookrs::task::Task;
    /// use task_hookrs::import::import;
    /// use libimagstore::store::{Store, FileLockEntry};
    ///
    /// if let Ok(task_hookrs_task) = import(stdin()) {
    ///     // Store is given at runtime
    ///     let task = task_hookrs_task.into_filelockentry(store);
    ///     println!("Task with uuid: {}", task.flentry.get_header().get("todo.uuid"));
    /// }
    /// ```
    fn into_filelockentry(self, store : &'a Store) -> Result<Task<'a>>;

}

impl<'a> IntoTask<'a> for TTask {

    fn into_filelockentry(self, store : &'a Store) -> Result<Task<'a>> {
        let uuid     = self.uuid();
        let store_id = ModuleEntryPath::new(format!("taskwarrior/{}", uuid)).into_storeid();

        match store.retrieve(store_id) {
            Err(e) => return Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e)))),
            Ok(mut fle) => {
                {
                    let mut header = fle.get_header_mut();
                    match header.read("todo") {
                        Ok(None) => {
                            if let Err(e) = header.set("todo", Value::Table(BTreeMap::new())) {
                                return Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e))))
                            }
                        }
                        Ok(Some(_)) => { }
                        Err(e) => {
                            return Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e))))
                        }
                    }

                    if let Err(e) = header.set("todo.uuid", Value::String(format!("{}",uuid))) {
                        return Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e))))
                    }
                }

                // If none of the errors above have returned the function, everything is fine
                Ok(Task::new(fle))
            }
        }
    }

}
