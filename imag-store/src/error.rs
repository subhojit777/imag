use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
/// Kind of store error
pub enum StoreErrorKind {
    BackendError,
    NoCommandlineCall,
    // maybe more
}

fn store_error_type_as_str(e: &StoreErrorKind) -> &'static str {
    match *e {
        StoreErrorKind::BackendError      => "Backend Error",
        StoreErrorKind::NoCommandlineCall => "No commandline call",
    }
}

impl Display for StoreErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", store_error_type_as_str(self)));
        Ok(())
    }

}

#[derive(Debug)]
pub struct StoreError {
    err_type: StoreErrorKind,
    cause: Option<Box<Error>>,
}

impl StoreError {

    ///Build a new StoreError from an StoreErrorKind, optionally with cause
    pub fn new(errtype: StoreErrorKind, cause: Option<Box<Error>>)
        -> StoreError
        {
            StoreError {
                err_type: errtype,
                cause: cause,
            }
        }

    /// Get the error type of this StoreError
    pub fn err_type(&self) -> StoreErrorKind {
        self.err_type
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
        store_error_type_as_str(&self.err_type)
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

