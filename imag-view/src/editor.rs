use libimagstore::store::FileLockEntry;
use libimagrt::runtime::Runtime;
use libimagentryview::builtin::editor::EditorView;
use libimagentryview::viewer::Viewer;
use libimagentryview::error::ViewError as VE;

pub struct Editor<'a> {
    rt: &'a Runtime<'a>,
    fle: &'a FileLockEntry<'a>,
}

impl<'a> Editor<'a> {
    pub fn new(rt: &'a Runtime, fle: &'a FileLockEntry) -> Editor<'a> {
        Editor{
            rt: rt,
            fle: fle,
        }
    }

    pub fn show(self) -> Result<(), VE> {
        EditorView::new(self.rt).view_entry(self.fle)
    }
}

