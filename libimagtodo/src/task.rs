use std::ops::Deref;

use uuid::Uuid;
use task_hookrs::task::Task as TTask;

use libimagstore::store::FileLockEntry;

use error::{TodoError, TodoErrorKind};

pub trait IntoTask<'a> {
    fn into_filelockentry(self) -> Result<Task<'a>, TodoError>;
}

#[derive(Debug)]
pub struct Task<'a> {
    flentry : FileLockEntry<'a>,
    //uuid : Uuid,
}
/*
impl<'a> From<TTask> for Task<'a> {
    fn from(ttask : TTask) -> Task<'a> {
        Task {
            flentry : {
            }
        }
    }
}
*/
impl<'a> IntoTask<'a> for TTask {
    fn into_filelockentry(self) -> Result<Task<'a>, TodoError> {
        Err(TodoError::new(TodoErrorKind::ConversionError, None))
    }
}
