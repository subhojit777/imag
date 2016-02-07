use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;
use std::error::Error;

use builtin::header::field_path::element::FieldPathElement;

#[derive(Debug)]
pub struct FieldPathParsingError {
    source: String,
    token: FieldPathElement
}

impl FieldPathParsingError {

    pub fn new(source: String, token: FieldPathElement) -> FieldPathParsingError {
        FieldPathParsingError { source: source, token: token }
    }
}

impl Display for FieldPathParsingError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "Failed to compile '{}', failed at: '{}'", self.source, self.token));
        Ok(())
    }

}

impl Error for FieldPathParsingError {

    fn description(&self) -> &str {
        &self.source[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}
