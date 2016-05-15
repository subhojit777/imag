use std::error::Error;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};

generate_error_types!(ViewError, ViewErrorKind,
    StoreError     => "Store error",
    NoVersion      => "No version specified",
    PatternError   => "Error in Pattern",
    GlobBuildError => "Could not build glob() Argument"
);

