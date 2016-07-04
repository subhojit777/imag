generate_error_module!(
    generate_error_types!(ViewError, ViewErrorKind,
        StoreError     => "Store error",
        NoVersion      => "No version specified",
        PatternError   => "Error in Pattern",
        GlobBuildError => "Could not build glob() Argument"
    );
);

pub use self::error::ViewError;
pub use self::error::ViewErrorKind;

