use std::error::Error;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};

generate_error_types!(CounterError, CounterErrorKind,
    StoreReadError          => "Store read error",
    StoreWriteError         => "Store write error",
    HeaderTypeError         => "Header type error",
    HeaderFieldMissingError => "Header field missing error"
);

