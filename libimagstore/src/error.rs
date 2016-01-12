use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;
use std::clone::Clone;

use std::io::Error as IOError;

pub struct StoreError {
    name: Option<&'static str>,
    expl: Option<&'static str>,
    cause: Option<Box<Error>>,
}

impl StoreError {

    pub fn new() -> StoreError {
        StoreError {
            name: None,
            expl:  None,
            cause: None,
        }
    }

    pub fn with_name(mut self, n: &'static str) -> StoreError {
        self.name = Some(n);
        self
    }

    pub fn with_expl(mut self, e: &'static str) -> StoreError {
        self.expl = Some(e);
        self
    }

    pub fn with_cause(mut self, e: Box<Error>) -> StoreError {
        self.cause = Some(e);
        self
    }

}

impl Debug for StoreError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{:?}]: {:?}, caused: {:?}", self.name, self.expl, self.cause));
        Ok(())
    }

}

impl Display for StoreError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]: {}",
                    self.name.unwrap_or("StoreError"),
                    self.expl.unwrap_or("")));
        Ok(())
    }

}

impl Error for StoreError {

    fn description(&self) -> &str {
        self.expl.unwrap_or("")
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

