use std::error::Error;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};

generate_error_types!(ViewError, ViewErrorKind,
    Unknown => "Unknown view error"
);

