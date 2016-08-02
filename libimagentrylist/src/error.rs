generate_error_module!(
    generate_error_types!(ListError, ListErrorKind,
        IOError        => "IO Error",
        FormatError    => "FormatError",
        EntryError     => "EntryError",
        IterationError => "IterationError",
        CLIError       => "No CLI subcommand for listing entries"
    );
);

pub use self::error::ListError;
pub use self::error::ListErrorKind;
pub use self::error::MapErrInto;

