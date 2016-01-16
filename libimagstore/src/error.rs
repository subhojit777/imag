use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::convert::From;

use std::io::Error as IOError;

#[derive(Clone, Copy)]
pub enum StoreErrorKind {
    IdNotFound,
    OutOfMemory,
    // maybe more
}

fn store_error_type_as_str(e: &StoreErrorKind) -> &'static str {
    match e {
        &StoreErrorKind::IdNotFound  => "ID not found",
        &StoreErrorKind::OutOfMemory => "Out of Memory",
    }
}

impl Debug for StoreErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{:?}", store_error_type_as_str(self)));
        Ok(())
    }

}

impl Display for StoreErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", store_error_type_as_str(self)));
        Ok(())
    }

}

pub struct StoreError {
    err_type: StoreErrorKind,
    cause: Option<Box<Error>>,
}

impl StoreError {

    pub fn new(errtype: StoreErrorKind, cause: Option<Box<Error>>)
        -> StoreError
    {
        StoreError {
            err_type: errtype,
            cause: cause,
        }
    }

    pub fn err_type(&self) -> StoreErrorKind {
        self.err_type.clone()
    }

}

impl Debug for StoreError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{:?}]: caused by: {:?}",
                    self.err_type, self.cause));
        Ok(())
    }

}

impl Display for StoreError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", store_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }

}

impl Error for StoreError {

    fn description(&self) -> &str {
        store_error_type_as_str(&self.err_type.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

