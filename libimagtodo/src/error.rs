use std::error::Error;
use std::clone::Clone;
use std::fmt::Error as FmtError;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

/// Enum of Error Types, as of now we have two:
/// * ConversionError: for Errors concerning conversion failures from task_hookrs::task::Task to
/// libimagtodo::task::Task. unused.
/// * StoreError: For Errors thrown by functions of the Store/structs relates to the Store
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TodoErrorKind {
    ConversionError,
    StoreError,
}

/// Maps a TodoErrorKind to a String
fn todo_error_type_as_str(e: &TodoErrorKind) -> &'static str {
    match e {
        &TodoErrorKind::ConversionError     => "Conversion Error",
        &TodoErrorKind::StoreError          => "Store Error",
    }
}

impl Display for TodoErrorKind {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", todo_error_type_as_str(self)));
        Ok(())
    }
}

/// Error struct for the imag-todo module
#[derive(Debug)]
pub struct TodoError {
    err_type : TodoErrorKind,
    cause : Option<Box<Error>>,
}

impl TodoError {
    /// Creates a new TodoError, with TodoErrorKind errtype and an optional cause
    pub fn new(errtype : TodoErrorKind, cause : Option<Box<Error>>) -> TodoError {
        TodoError {
            err_type : errtype,
            cause : cause,
        }
    }
    /// Returns the error type (TodoErrorKind)
    pub fn err_type(&self) -> TodoErrorKind {
        self.err_type.clone()
    }
}

impl Display for TodoError {
    fn fmt(&self, fmt : &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", todo_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }
}

impl Error for TodoError {
    fn description(&self) -> &str {
        todo_error_type_as_str(&self.err_type.clone())
    }
    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }
}
