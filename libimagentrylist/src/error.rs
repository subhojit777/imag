use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Display, Formatter};

/**
 * Kind of error
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ListErrorKind {
    FormatError,
    EntryError,
    IterationError,
    CLIError,
}

fn counter_error_type_as_str(err: &ListErrorKind) -> &'static str{
    match err {
        &ListErrorKind::FormatError    => "FormatError",
        &ListErrorKind::EntryError     => "EntryError",
        &ListErrorKind::IterationError => "IterationError",
        &ListErrorKind::CLIError       => "No CLI subcommand for listing entries",
    }
}

impl Display for ListErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", counter_error_type_as_str(self)));
        Ok(())
    }

}

/**
 * Store error type
 */
#[derive(Debug)]
pub struct ListError {
    err_type: ListErrorKind,
    cause: Option<Box<Error>>,
}

impl ListError {

    /**
     * Build a new ListError from an ListErrorKind, optionally with cause
     */
    pub fn new(errtype: ListErrorKind, cause: Option<Box<Error>>) -> ListError {
            ListError {
                err_type: errtype,
                cause: cause,
            }
        }

    /**
     * Get the error type of this ListError
     */
    pub fn err_type(&self) -> ListErrorKind {
        self.err_type.clone()
    }

}

impl Display for ListError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", counter_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }

}

impl Error for ListError {

    fn description(&self) -> &str {
        counter_error_type_as_str(&self.err_type.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

