generate_error_imports!();
use std::convert::Into;

generate_error_types!(HookError, HookErrorKind,
    HookExecutionError  => "Hook exec error",
    AccessTypeViolation => "Hook access type violation"
);

pub trait IntoHookError {
    fn into_hookerror(self) -> HookError;
    fn into_hookerror_with_cause(self, cause: Box<Error>) -> HookError;
}

impl Into<HookError> for (HookErrorKind, Box<Error>) {

    fn into(self) -> HookError {
        HookError::new(self.0, Some(self.1))
    }

}

