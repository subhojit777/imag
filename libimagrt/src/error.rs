use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;
use std::io::Error as IOError;

generate_error_types!(RuntimeError, RuntimeErrorKind,
    Instantiate        => "Could not instantiate",
    IOError            => "IO Error",
    ProcessExitFailure => "Process exited with failure"
);

impl From<IOError> for RuntimeError {

    fn from(ioe: IOError) -> RuntimeError {
        RuntimeError::new(RuntimeErrorKind::IOError, Some(Box::new(ioe)))
    }

}

