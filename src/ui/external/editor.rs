use std::ops::Drop;

use std::path::PathBuf;
use std::fs::File;
use std::error::Error;

use runtime::Runtime;

/*
 * A Temporary file in /tmp where the editor is launched on, so we can grap the contents and put
 * into the store
 */
pub struct TempFile {
    path: Option<PathBuf>,
    file: Option<File>,
}

impl TempFile {

    pub fn new(rt: &Runtime) -> TempFile {
        debug!("Building new TempFile");
        unimplemented!()
    }

    pub fn edit(&mut self, editor: Option<String>) -> TempFile {
        debug!("Editing TempFile");
        unimplemented!()
    }

    pub fn content(&self) -> Result<String, TempFileError> {
        debug!("Fetching content of TempFile");
        unimplemented!()
    }

}

/*
 * Implement Drop, so we ensure to remove the tempfile
 *
 * (is this neccessary if we use a real tempfile)
 */
impl Drop for TempFile {

    fn drop(&mut self) {
        unimplemented!()
    }

}


pub struct TempFileError {
    cause: Option<Box<Error>>,
}
