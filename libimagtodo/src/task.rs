use std::ops::Deref;

use task_hookrs::task::Task as TTask;

use libimagstore::store::FileLockEntry;

pub struct Task {
    uuid : str,
}

impl Deref<FileLockEntry> for Task {

}
