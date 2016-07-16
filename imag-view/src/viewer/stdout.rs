use std::io::{Stdout, stdout};

use toml::encode_str;

use viewer::{ViewInformation, Viewer};

pub struct StdoutViewer {
    out: Stdout,
}

impl StdoutViewer {

    pub fn new() -> StdoutViewer {
        StdoutViewer { out: stdout() }
    }

}

impl Viewer for StdoutViewer {

    fn view(&self, vi: ViewInformation) {
        if vi.view_copy {
            unimplemented!();
        }

        if vi.view_header {
            debug!("Going to display header: {:?}", vi.entry.get_header().header());
            println!("{}", encode_str(vi.entry.get_header().header()));
        }

        if vi.view_content {
            println!("{}", vi.entry.get_content());
        }

        if vi.view_copy && !vi.keep_copy {
            unimplemented!()
        }
    }

}
