use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::convert::From;

/**
 * Kind of error
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CounterErrorKind {
    StoreReadError,
    StoreWriteError,
    HeaderTypeError,
    HeaderFieldMissingError,
}

fn counter_error_type_as_str(e: &CounterErrorKind) -> &'static str {
    match e {
        &CounterErrorKind::StoreReadError  => "Store read error",
        &CounterErrorKind::StoreWriteError => "Store write error",
        &CounterErrorKind::HeaderTypeError => "Header type error",
        &CounterErrorKind::HeaderFieldMissingError => "Header field missing error",
    }
}

impl Display for CounterErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", counter_error_type_as_str(self)));
        Ok(())
    }

}

/**
 * Store error type
 */
#[derive(Debug)]
pub struct CounterError {
    err_type: CounterErrorKind,
    cause: Option<Box<Error>>,
}

impl CounterError {

    /**
     * Build a new CounterError from an CounterErrorKind, optionally with cause
     */
    pub fn new(errtype: CounterErrorKind, cause: Option<Box<Error>>)
        -> CounterError
        {
            CounterError {
                err_type: errtype,
                cause: cause,
            }
        }

    /**
     * Get the error type of this CounterError
     */
    pub fn err_type(&self) -> CounterErrorKind {
        self.err_type.clone()
    }

}

impl Display for CounterError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", counter_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }

}

impl Error for CounterError {

    fn description(&self) -> &str {
        counter_error_type_as_str(&self.err_type.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

