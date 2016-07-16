use libimagstore::store::Entry;

use toml::encode_str;

use viewer::Viewer;
use result::Result;

pub struct StdoutViewer {
    view_header: bool,
    view_content: bool,
}

impl StdoutViewer {

    pub fn new(view_header: bool, view_content: bool) -> StdoutViewer {
        StdoutViewer {
            view_header: view_header,
            view_content: view_content,
        }
    }

}

impl Viewer for StdoutViewer {

    fn view_entry(&self, e: &Entry) -> Result<()> {
        if self.view_header {
            println!("{}", encode_str(e.get_header().header()));
        }

        if self.view_content {
            println!("{}", e.get_content());
        }

        Ok(())
    }

}
