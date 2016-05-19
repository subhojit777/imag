generate_error_module!(
    generate_error_types!(StoreError, StoreErrorKind,
        BackendError      => "Backend Error",
        NoCommandlineCall => "No commandline call"
    );
);

pub use self::error::StoreError;
pub use self::error::StoreErrorKind;

