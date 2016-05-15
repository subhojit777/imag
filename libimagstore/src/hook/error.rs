use std::error::Error;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};
use std::convert::Into;

/**
 * Kind of error
 */
#[derive(Clone, Copy, Debug)]
pub enum HookErrorKind {
    HookExecutionError,
    AccessTypeViolation,
}

pub trait IntoHookError {
    fn into_hookerror(self) -> HookError;
    fn into_hookerror_with_cause(self, cause: Box<Error>) -> HookError;
}

impl Into<HookError> for HookErrorKind {

    fn into(self) -> HookError {
        HookError::new(self, None)
    }

}

impl Into<HookError> for (HookErrorKind, Box<Error>) {

    fn into(self) -> HookError {
        HookError::new(self.0, Some(self.1))
    }

}

fn hook_error_type_as_str(e: &HookErrorKind) -> &'static str {
    match *e {
        HookErrorKind::HookExecutionError  => "Hook exec error",
        HookErrorKind::AccessTypeViolation => "Hook access type violation",
    }
}

impl Display for HookErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", hook_error_type_as_str(self)));
        Ok(())
    }

}

/**
 * Error type
 */
#[derive(Debug)]
pub struct HookError {
    err_type: HookErrorKind,
    cause: Option<Box<Error>>,
}

impl HookError {

    /**
     * Build a new HookError from an HookErrorKind, optionally with cause
     */
    pub fn new(errtype: HookErrorKind, cause: Option<Box<Error>>)
        -> HookError
        {
            HookError {
                err_type: errtype,
                cause: cause,
            }
        }

    /**
     * Get the error type of this HookError
     */
    pub fn err_type(&self) -> HookErrorKind {
        self.err_type
    }

}

impl Display for HookError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", hook_error_type_as_str(&self.err_type)));
        Ok(())
    }

}

impl Error for HookError {

    fn description(&self) -> &str {
        hook_error_type_as_str(&self.err_type)
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}


