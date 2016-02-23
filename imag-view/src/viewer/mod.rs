pub mod stdout;

use libimagstore::store::Entry;

#[derive(Debug)]
pub struct ViewInformation {
    pub entry: Entry,
    pub view_header: bool,
    pub view_content: bool,
    pub view_copy: bool,
    pub keep_copy: bool,
}

pub trait Viewer {
    fn view(&self, vi: ViewInformation);
}

