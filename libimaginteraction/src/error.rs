generate_error_module!(
    generate_error_types!(InteractionError, InteractionErrorKind,
        Unknown => "Unknown Error"
    );
);

pub use self::error::InteractionError;
pub use self::error::InteractionErrorKind;

