use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::convert::From;

use std::io::Error as IOError;

#[derive(Clone)]
pub enum StoreErrorType {
    Unknown,
    IdNotFound,
    OutOfMemory,
    // maybe more
}

impl From<StoreErrorType> for String {

    fn from(e: StoreErrorType) -> String {
        String::from(&e)
    }

}

impl<'a> From<&'a StoreErrorType> for String {

    fn from(e: &'a StoreErrorType) -> String {
        match e {
            &StoreErrorType::Unknown     => String::from("<Unknown>"),
            &StoreErrorType::IdNotFound  => String::from("ID not found"),
            &StoreErrorType::OutOfMemory => String::from("Out of Memory"),
        }
    }

}

impl Debug for StoreErrorType {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        let s : String = self.into();
        try!(write!(fmt, "{:?}", s));
        Ok(())
    }

}

impl Display for StoreErrorType {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        let s : String = self.into();
        try!(write!(fmt, "{}", s));
        Ok(())
    }

}

pub struct StoreError {
    err_type: StoreErrorType,
    expl: Option<&'static str>,
    cause: Option<Box<Error>>,
}

impl StoreError {

    pub fn new() -> StoreError {
        StoreError {
            err_type: StoreErrorType::Unknown,
            expl:  None,
            cause: None,
        }
    }

    pub fn err_type(&self) -> StoreErrorType {
        self.err_type.clone()
    }

    pub fn with_type(mut self, t: StoreErrorType) -> StoreError {
        self.err_type = t;
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
        try!(write!(fmt, "[{:?}]: {:?}, caused: {:?}",
                    self.err_type, self.expl, self.cause));
        Ok(())
    }

}

impl Display for StoreError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        let e : String = self.err_type.clone().into();
        try!(write!(fmt, "[{}]: {}",
                    e,
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

