generate_error_module!(
    generate_error_types!(ViewError, ViewErrorKind,
        Unknown              => "Unknown view error",
        GlobError            => "Error while glob()ing",
        PatternError         => "Error in glob() pattern",
        PatternBuildingError => "Could not build glob() pattern"
    );
);

pub use self::error::ViewError;
pub use self::error::ViewErrorKind;
pub use self::error::MapErrInto;
