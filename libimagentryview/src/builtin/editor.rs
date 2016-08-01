use libimagstore::store::Entry;
use libimagrt::runtime::Runtime;
use libimagrt::edit::edit_in_tmpfile;

use viewer::Viewer;
use result::Result;
use error::ViewErrorKind as VEK;
use error::ViewError as VE;

pub struct EditorView<'a>(&'a Runtime<'a>);

impl<'a> EditorView<'a> {
    pub fn new(rt: &'a Runtime) -> EditorView<'a> {
        EditorView(rt)
    }
}

impl<'a> Viewer for EditorView<'a> {
    fn view_entry(&self, e: &Entry) -> Result<()> {
        let mut entry = e.to_str().clone().to_string();
        edit_in_tmpfile(self.0, &mut entry)
            .map_err(|e| VE::new(VEK::ViewError, Some(Box::new(e))))
    }
}

