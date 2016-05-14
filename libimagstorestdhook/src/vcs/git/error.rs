generate_error_module!(
    generate_error_types!(GitHookError, GitHookErrorKind,
        MkRepo => "Repository creation error"
    );
);

pub use self::error::GitHookError;
pub use self::error::GitHookErrorKind;

