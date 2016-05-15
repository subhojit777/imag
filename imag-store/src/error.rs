use std::error::Error;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};

generate_error_types!(StoreError, StoreErrorKind,
    BackendError      => "Backend Error",
    NoCommandlineCall => "No commandline call"
);

