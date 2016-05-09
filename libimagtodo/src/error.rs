use std::error::Error;
use std::clone::Clone;
use std::fmt::Error as FmtError;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TodoErrorKind {
    ConversionError,
}

fn todo_error_type_as_str(e: &TodoErrorKind) -> &'static str {
    match e {
        &TodoErrorKind::ConversionError     => "Conversion Error",
    }
}

impl Display for TodoErrorKind {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", todo_error_type_as_str(self)));
        Ok(())
    }
}

#[derive(Debug)]
pub struct TodoError {
    err_type : TodoErrorKind,
    cause : Option<Box<Error>>,
}

impl TodoError {
    pub fn new(errtype : TodoErrorKind, cause : Option<Box<Error>>) -> TodoError {
        TodoError {
            err_type : errtype,
            cause : cause,
        }
    }
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



