use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TagErrorKind {
    TagTypeError,
    HeaderReadError,
}

fn tag_error_type_as_str(e: &TagErrorKind) -> &'static str {
    match e {
        &TagErrorKind::TagTypeError => "Entry Header Tag Type wrong",
        &TagErrorKind::HeaderReadError => "Error while reading entry header",
    }
}

impl Display for TagErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", tag_error_type_as_str(self)));
        Ok(())
    }

}

#[derive(Debug)]
pub struct TagError {
    kind: TagErrorKind,
    cause: Option<Box<Error>>,
}

impl TagError {

    pub fn new(errtype: TagErrorKind, cause: Option<Box<Error>>) -> TagError {
        TagError {
            kind: errtype,
            cause: cause,
        }
    }

}

impl Display for TagError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", tag_error_type_as_str(&self.kind.clone())));
        Ok(())
    }

}

impl Error for TagError {

    fn description(&self) -> &str {
        tag_error_type_as_str(&self.kind.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

