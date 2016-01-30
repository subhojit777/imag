use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RuntimeErrorKind {
    Instantiate,

    // more?
}

#[derive(Debug)]
pub struct RuntimeError {
    kind: RuntimeErrorKind,
    cause: Option<Box<Error>>,
}

impl RuntimeError {

    pub fn new(kind: RuntimeErrorKind, cause: Option<Box<Error>>) -> RuntimeError {
        RuntimeError {
            kind: kind,
            cause: cause,
        }
    }

}

fn runtime_error_kind_as_str(e: &RuntimeErrorKind) -> &'static str {
    match e {
        &RuntimeErrorKind::Instantiate => "Could not instantiate",
    }
}

impl Display for RuntimeError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", runtime_error_kind_as_str(&self.kind)));
        Ok(())
    }

}

impl Error for RuntimeError {

    fn description(&self) -> &str {
        runtime_error_kind_as_str(&self.kind)
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

