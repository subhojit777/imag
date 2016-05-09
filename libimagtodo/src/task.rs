use std::ops::Deref;
use toml::Value;

use task_hookrs::task::Task as TTask;

use libimagstore::store::{FileLockEntry, Store};
use libimagstore::storeid::IntoStoreId;
use module_path::ModuleEntryPath;

use error::{TodoError, TodoErrorKind};

/// Task struct containing a `FileLockEntry`
#[derive(Debug)]
pub struct Task<'a> {
    flentry : FileLockEntry<'a>,
}

impl<'a> Task<'a> {
    /// Concstructs a new `Task` with a `FileLockEntry`
    pub fn new(fle : FileLockEntry<'a>) -> Task<'a> {
        Task {
            flentry : fle
        }
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
    fn into_filelockentry(self, store : &'a Store) -> Result<Task<'a>, TodoError>;
}
impl<'a> IntoTask<'a> for TTask {
    fn into_filelockentry(self, store : &'a Store) -> Result<Task<'a>, TodoError> {
        let uuid = self.uuid();
        let store_id = ModuleEntryPath::new(format!("taskwarrior/{}", uuid)).into_storeid();
        match store.retrieve(store_id) {
            Err(e)      => Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e)))),
            Ok(mut fle)     => {
                match fle.get_header_mut().set("todo.uuid", Value::String(format!("{}", uuid))) {
                    Err(e) => Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e)))),
                    Ok(_) => Ok(Task { flentry : fle })
                }
            },
        }
    }
}
