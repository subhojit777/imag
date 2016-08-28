generate_error_module!(
    generate_error_types!(EntrySelectError, EntrySelectErrorKind,
        CLIError        => "Error on commandline",
        IdMissingError  => "Commandline: ID missing",
        StoreIdParsingError => "Error while parsing StoreId",
        IdSelectingError => "Error while selecting id"
    );
);

pub use self::error::EntrySelectError;
pub use self::error::EntrySelectErrorKind;
pub use self::error::MapErrInto;

