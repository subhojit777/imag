generate_error_module!(
    generate_error_types!(MailError, MailErrorKind,
        IOError => "IO Error"
    );
);

pub use self::error::MailError;
pub use self::error::MailErrorKind;
pub use self::error::MapErrInto;

