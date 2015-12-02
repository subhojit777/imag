use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::result::Result;
use std::path::{Path, PathBuf};
use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
#[derive(Clone)]
// #[derive(Display)]
pub enum FileIDType {
    UUID,
}

#[derive(Clone)]
pub struct FileID {
    id: Option<String>,
    id_type: FileIDType,
}

impl FileID {

    pub fn new(id_type: FileIDType, id: String) -> FileID {
        FileID {
            id: Some(id),
            id_type: id_type,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.id.is_some()
    }

}

impl Debug for FileID {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "FileID[{:?}]: {:?}",
               self.id_type,
               self.id);
        Ok(())
    }

}

impl Display for FileID {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "FileID[{:?}]: {:?}",
               self.id_type,
               self.id);
        Ok(())
    }

}

impl Into<String> for FileID {

    fn into(self) -> String {
        if let Some(id) = self.id {
            id.clone()
        } else {
            String::from("INVALID")
        }
    }

}

impl From<String> for FileID {

    fn from(s: String) -> FileID {
        unimplemented!()
    }

}

impl<'a> From<&'a String> for FileID {

    fn from(s: &'a String) -> FileID {
        unimplemented!()
    }

}

impl From<PathBuf> for FileID {

    fn from(s: PathBuf) -> FileID {
        unimplemented!()
    }

}

impl<'a> From<&'a PathBuf> for FileID {

    fn from(s: &'a PathBuf) -> FileID {
        unimplemented!()
    }

}

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

