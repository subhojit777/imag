use std::ops::Deref;

use task_hookrs::task::Task as TTask;

use libimagstore::store::FileLockEntry;

#[derive(Debug)]
pub struct Task<'a> {
    flentry : FileLockEntry<'a>,
}

impl<'a> From<TTask> for Task<'a> {
    fn from(ttask : TTask) -> Task<'a> {
        Task {
            flentry : {
            }
        }
    }
}

