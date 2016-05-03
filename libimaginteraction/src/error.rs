use std::error::Error;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};

/**
 * Kind of error
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InteractionErrorKind {
    Unknown
}

fn interaction_error_type_as_str(e: &InteractionErrorKind) -> &'static str {
    match *e {
        InteractionErrorKind::Unknown => "Unknown Error",
    }
}

impl Display for InteractionErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", interaction_error_type_as_str(self)));
        Ok(())
    }

}

#[derive(Debug)]
pub struct InteractionError {
    err_type: InteractionErrorKind,
    cause: Option<Box<Error>>,
}

impl InteractionError {

    /**
     * Build a new InteractionError from an InteractionErrorKind, optionally with cause
     */
    pub fn new(errtype: InteractionErrorKind, cause: Option<Box<Error>>)
        -> InteractionError
        {
            InteractionError {
                err_type: errtype,
                cause: cause,
            }
        }

    /**
     * Get the error type of this InteractionError
     */
    pub fn err_type(&self) -> InteractionErrorKind {
        self.err_type
    }

}

impl Display for InteractionError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", interaction_error_type_as_str(&self.err_type)));
        Ok(())
    }

}

impl Error for InteractionError {

    fn description(&self) -> &str {
        interaction_error_type_as_str(&self.err_type)
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

