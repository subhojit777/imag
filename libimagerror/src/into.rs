use std::error::Error;

/// Trait to help converting Error kinds into Error instances
pub trait IntoError {
    type Target: Error;

    /// Convert the type into an error with no cause
    fn into_error(self) -> Self::Target;

    /// Convert the type into an error with cause
    fn into_error_with_cause(self, cause: Box<Error>) -> Self::Target;

}

