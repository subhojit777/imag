generate_error_imports!();

generate_error_types!(ViewError, ViewErrorKind,
    StoreError     => "Store error",
    NoVersion      => "No version specified",
    PatternError   => "Error in Pattern",
    GlobBuildError => "Could not build glob() Argument"
);

