use std::ops::Drop;

use std::path::PathBuf;
use std::fs::File;
use std::error::Error;

use std::fmt::{Debug, Display, Formatter};
use std::fmt;

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
    desc: &'static str,
    caused_by: Option<Box<Error>>,
}

impl TempFileError {

    pub fn new(s: &'static str) -> TempFileError {
        TempFileError {
            desc: s,
            caused_by: None,
        }
    }

    pub fn with_cause(s: &'static str, c: Box<Error>) -> TempFileError {
        TempFileError {
            desc: s,
            caused_by: Some(c),
        }
    }

}

impl Display for TempFileError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "TempFileError: '{}'", self.desc);
        Ok(())
    }
}

impl Debug for TempFileError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "TempFileError: '{}'", self.desc);

        if let Some(ref e) = self.caused_by {
            write!(fmt, "  cause: {:?}\n\n", e);
        }

        Ok(())
    }
}

impl Error for TempFileError {

    fn description(&self) -> &str {
        self.desc
    }

    fn cause(&self) -> Option<&Error> {
        self.caused_by.as_ref().map(|e| &**e)
    }

}

