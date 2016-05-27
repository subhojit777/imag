generate_error_module!(
    generate_error_types!(HookError, HookErrorKind,
        HookExecutionError  => "Hook exec error",
        AccessTypeViolation => "Hook access type violation"
    );
);

pub use self::error::HookError;
pub use self::error::HookErrorKind;

