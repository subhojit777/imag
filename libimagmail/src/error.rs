generate_error_module!(
    generate_error_types!(MailError, MailErrorKind,
        RefCreationError => "Error creating a reference to a file/directory",
        MailParsingError => "Error while parsing mail",
        IOError => "IO Error"
    );
);

pub use self::error::MailError;
pub use self::error::MailErrorKind;
pub use self::error::MapErrInto;

