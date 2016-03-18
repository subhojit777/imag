pub mod stdout;

use libimagstore::store::FileLockEntry;

pub struct ViewInformation<'a> {
    pub entry: FileLockEntry<'a>,
    pub view_header: bool,
    pub view_content: bool,
    pub view_copy: bool,
    pub keep_copy: bool,
}

pub trait Viewer {
    fn view(&self, vi: ViewInformation);
}

