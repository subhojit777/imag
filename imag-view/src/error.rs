use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::convert::From;

/**
 * Kind of store error
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewErrorKind {
    StoreError,
    NoVersion,
    PatternError,
    GlobBuildError,
}

fn view_error_type_as_str(e: &ViewErrorKind) -> &'static str {
    match e {
        &ViewErrorKind::StoreError => "Store error",
        &ViewErrorKind::NoVersion => "No version specified",
        &ViewErrorKind::PatternError => "Error in Pattern",
        &ViewErrorKind::GlobBuildError => "Could not build glob() Argument",
    }
}

impl Display for ViewErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", view_error_type_as_str(self)));
        Ok(())
    }

}

/**
 * View error type
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
        try!(write!(fmt, "[{}]", view_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }

}

impl Error for ViewError {

    fn description(&self) -> &str {
        view_error_type_as_str(&self.err_type.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

