use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::result::Result;
use std::path::{Path, PathBuf};

pub type FileID = String;

pub struct FileIDError {
    summary: String,
    descrip: String,
}

impl FileIDError {

    pub fn new(s: String, d: String) -> FileIDError {
        FileIDError {
            summary: s,
            descrip: d,
        }
    }

}

impl<'a> Error for FileIDError {

    fn description(&self) -> &str {
        &self.summary[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl<'a> Debug for FileIDError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "FileIDError: '{}'\n{}", self.summary, self.descrip);
        Ok(())
    }

}

impl<'a> Display for FileIDError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "FileIDError: '{}'", self.summary);
        Ok(())
    }

}

pub type FileIDResult = Result<FileID, FileIDError>;

pub fn from_path_string(s: &String) -> FileIDResult {
    unimplemented!()
}

pub fn from_path(p: &Path) -> FileIDResult {
    unimplemented!()
}

pub fn from_pathbuf(p: &PathBuf) -> FileIDResult {
    from_path(p.as_path())
}

