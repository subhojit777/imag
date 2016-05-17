use std::error::Error;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};

generate_error_types!(ListError, ListErrorKind,
    FormatError    => "FormatError",
    EntryError     => "EntryError",
    IterationError => "IterationError",
    CLIError       => "No CLI subcommand for listing entries"
);

