use std::io::{Stdout, stdout};

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
        unimplemented!()
    }

}
