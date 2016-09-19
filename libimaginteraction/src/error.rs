generate_error_module!(
    generate_error_types!(InteractionError, InteractionErrorKind,
        Unknown             => "Unknown Error",
        CLIError            => "Error on commandline",
        IdMissingError      => "Commandline: ID missing",
        StoreIdParsingError => "Error while parsing StoreId",
        IdSelectingError    => "Error while selecting id"
    );
);

pub use self::error::InteractionError;
pub use self::error::InteractionErrorKind;
pub use self::error::MapErrInto;

