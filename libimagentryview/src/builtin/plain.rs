use libimagstore::store::Entry;

use viewer::Viewer;
use result::Result;

pub struct PlainViewer {
    show_header: bool
}

impl PlainViewer {

    pub fn new(show_header: bool) -> PlainViewer {
        PlainViewer {
            show_header: show_header,
        }
    }

}

impl Viewer for PlainViewer {

    fn view_entry(&self, e: &Entry) -> Result<()> {
        if self.show_header {
            println!("{}", e.get_header().header());
        }
        println!("{}", e.get_content());
        Ok(())
    }

}
