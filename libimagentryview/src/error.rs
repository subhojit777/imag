generate_error_module!(
    generate_error_types!(ViewError, ViewErrorKind,
        Unknown => "Unknown view error"
    );
);

pub use self::error::ViewError;
pub use self::error::ViewErrorKind;

