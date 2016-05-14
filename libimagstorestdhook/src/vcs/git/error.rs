use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Display, Formatter};

/**
 * Kind of error
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GitHookErrorKind {
}

fn githook_error_type_as_str(e: &GitHookErrorKind) -> &'static str {
    match *e {
        _ => "",
    }
}

impl Display for GitHookErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", githook_error_type_as_str(self)));
        Ok(())
    }

}

/**
 * Store error type
 */
#[derive(Debug)]
pub struct GitHookError {
    err_type: GitHookErrorKind,
    cause: Option<Box<Error>>,
}

impl GitHookError {

    /**
     * Build a new GitHookError from an GitHookErrorKind, optionally with cause
     */
    pub fn new(errtype: GitHookErrorKind, cause: Option<Box<Error>>)
        -> GitHookError
        {
            GitHookError {
                err_type: errtype,
                cause: cause,
            }
        }

    /**
     * Get the error type of this GitHookError
     */
    pub fn err_type(&self) -> GitHookErrorKind {
        self.err_type.clone()
    }

}

impl Display for GitHookError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", githook_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }

}

impl Error for GitHookError {

    fn description(&self) -> &str {
        githook_error_type_as_str(&self.err_type.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

