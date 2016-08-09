generate_error_module!(
    generate_error_types!(EditError, EditErrorKind,
        IOError             => "IO Error"
    );
);

pub use self::error::EditError;
pub use self::error::EditErrorKind;
pub use self::error::MapErrInto;

