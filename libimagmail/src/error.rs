generate_error_module!(
    generate_error_types!(MailError, MailErrorKind,
        RefCreationError => "Error creating a reference to a file/directory",
        RefHandlingError => "Error while handling the internal reference object",
        MailParsingError => "Error while parsing mail",

        FetchByHashError => "Error fetching mail from Store by hash",
        FetchError       => "Error fetching mail from Store",
        IOError => "IO Error"
    );
);

pub use self::error::MailError;
pub use self::error::MailErrorKind;
pub use self::error::MapErrInto;

