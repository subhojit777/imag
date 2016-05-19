generate_error_imports!();

generate_error_types!(StoreError, StoreErrorKind,
    BackendError      => "Backend Error",
    NoCommandlineCall => "No commandline call"
);

