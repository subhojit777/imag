use std::error::Error;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};

generate_error_types!(TagError, TagErrorKind,
    TagTypeError     => "Entry Header Tag Type wrong",
    HeaderReadError  => "Error while reading entry header",
    HeaderWriteError => "Error while writing entry header",
    NotATag          => "String is not a tag"
);

