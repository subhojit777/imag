use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Display, Formatter};

/**
 * Kind of error
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewErrorKind {
}

fn counter_error_type_as_str(e: &ViewErrorKind) -> &'static str {
    match e {
        _ => "",
    }
}

impl Display for ViewErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", counter_error_type_as_str(self)));
        Ok(())
    }

}

/**
 * Store error type
 */
#[derive(Debug)]
pub struct ViewError {
    err_type: ViewErrorKind,
    cause: Option<Box<Error>>,
}

impl ViewError {

    /**
     * Build a new ViewError from an ViewErrorKind, optionally with cause
     */
    pub fn new(errtype: ViewErrorKind, cause: Option<Box<Error>>)
        -> ViewError
        {
            ViewError {
                err_type: errtype,
                cause: cause,
            }
        }

    /**
     * Get the error type of this ViewError
     */
    pub fn err_type(&self) -> ViewErrorKind {
        self.err_type.clone()
    }

}

impl Display for ViewError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", counter_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }

}

impl Error for ViewError {

    fn description(&self) -> &str {
        counter_error_type_as_str(&self.err_type.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

