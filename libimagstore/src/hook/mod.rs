use std::fmt::Debug;

use toml::Value;

use self::error::HookError;
use store::FileLockEntry;

pub type HookResult<T> = Result<T, HookError>;

pub trait Configureable {
    fn set_config(&mut self, cfg: Value);
}

pub trait MutableHookDataAccessor : Send + Sync {
    fn access_mut(&self, &mut FileLockEntry) -> HookResult<()>;
}

pub trait NonMutableHookDataAccessor : Send + Sync {
    fn access(&self, &FileLockEntry) -> HookResult<()>;
}

pub enum HookDataAccessor {
    MutableAccess(Box<MutableHookDataAccessor>),
    NonMutableAccess(Box<NonMutableHookDataAccessor>),
}

pub trait HookDataAccessorProvider {
    fn accessor(&self) -> Box<HookDataAccessor>;
}

pub trait Hook : Configureable + Debug + Send + Sync {
}

pub mod read {
    use storeid::StoreId;
    use store::FileLockEntry;
    use super::HookDataAccessorProvider;
    use super::HookResult;
    use super::Hook;

    pub trait PreReadHook : Hook {
        fn pre_read(&self, &StoreId) -> HookResult<()>;
    }

    pub trait PostReadHook : Hook + HookDataAccessorProvider {
    }

}

pub mod create {
    use storeid::StoreId;
    use store::FileLockEntry;
    use super::HookDataAccessorProvider;
    use super::HookResult;
    use super::Hook;

    pub trait PreCreateHook : Hook {
        fn pre_create(&self, &StoreId) -> HookResult<()>;
    }

    pub trait PostCreateHook : Hook + HookDataAccessorProvider {
    }

}

pub mod retrieve {
    use storeid::StoreId;
    use store::FileLockEntry;
    use super::HookDataAccessorProvider;
    use super::HookResult;
    use super::Hook;

    pub trait PreRetrieveHook : Hook {
        fn pre_retrieve(&self, &StoreId) -> HookResult<()>;
    }

    pub trait PostRetrieveHook : Hook + HookDataAccessorProvider {
    }
}

pub mod update {
    use store::FileLockEntry;
    use super::HookDataAccessorProvider;
    use super::HookResult;
    use super::Hook;

    pub trait PreUpdateHook : Hook {
        fn pre_update(&self, &FileLockEntry) -> HookResult<()>;
    }

    pub trait PostUpdateHook : Hook + HookDataAccessorProvider {
    }
}

pub mod delete {
    use storeid::StoreId;
    use store::FileLockEntry;
    use super::HookDataAccessorProvider;
    use super::HookResult;
    use super::Hook;

    pub trait PreDeleteHook : Hook {
        fn pre_delete(&self, &StoreId) -> HookResult<()>;
    }

    pub trait PostDeleteHook : Hook {
        fn post_delete(&self, &StoreId) -> HookResult<()>;
    }
}

pub mod error {
    use std::error::Error;
    use std::fmt::Error as FmtError;
    use std::clone::Clone;
    use std::fmt::{Display, Formatter};
    use std::convert::Into;

    /**
     * Kind of error
     */
    #[derive(Clone, Copy, Debug)]
    pub enum HookErrorKind {
        Pre(PreHookErrorKind),
        Post(PostHookErrorKind)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PreHookErrorKind {
        // ...
    }

    impl Into<HookErrorKind> for PreHookErrorKind {
        fn into(self) -> HookErrorKind {
            HookErrorKind::Pre(self)
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PostHookErrorKind {
        // ...
    }

    impl Into<HookErrorKind> for PostHookErrorKind {
        fn into(self) -> HookErrorKind {
            HookErrorKind::Post(self)
        }
    }

    pub trait IntoHookError {
        fn into_hookerror(self) -> HookError;
        fn into_hookerror_with_cause(self, cause: Box<Error>) -> HookError;
    }

    impl Into<HookError> for HookErrorKind {

        fn into(self) -> HookError {
            HookError::new(self, None)
        }

    }

    impl Into<HookError> for (HookErrorKind, Box<Error>) {

        fn into(self) -> HookError {
            HookError::new(self.0, Some(self.1))
        }

    }

    impl Into<HookError> for PreHookErrorKind {

        fn into(self) -> HookError {
            HookError::new(HookErrorKind::Pre(self), None)
        }

    }

    impl Into<HookError> for (PreHookErrorKind, Box<Error>) {

        fn into(self) -> HookError {
            HookError::new(HookErrorKind::Pre(self.0), Some(self.1))
        }

    }

    impl Into<HookError> for PostHookErrorKind {

        fn into(self) -> HookError {
            HookError::new(HookErrorKind::Post(self), None)
        }

    }

    impl Into<HookError> for (PostHookErrorKind, Box<Error>) {

        fn into(self) -> HookError {
            HookError::new(HookErrorKind::Post(self.0), Some(self.1))
        }

    }

    fn hook_error_type_as_str(e: &HookErrorKind) -> &'static str {
        match e {
            _ => "",
        }
    }

    impl Display for HookErrorKind {

        fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
            try!(write!(fmt, "{}", hook_error_type_as_str(self)));
            Ok(())
        }

    }

    /**
     * Error type
     */
    #[derive(Debug)]
    pub struct HookError {
        err_type: HookErrorKind,
        cause: Option<Box<Error>>,
    }

    impl HookError {

        /**
         * Build a new HookError from an HookErrorKind, optionally with cause
         */
        pub fn new(errtype: HookErrorKind, cause: Option<Box<Error>>)
            -> HookError
            {
                HookError {
                    err_type: errtype,
                    cause: cause,
                }
            }

        /**
         * Get the error type of this HookError
         */
        pub fn err_type(&self) -> HookErrorKind {
            self.err_type.clone()
        }

    }

    impl Display for HookError {

        fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
            try!(write!(fmt, "[{}]", hook_error_type_as_str(&self.err_type.clone())));
            Ok(())
        }

    }

    impl Error for HookError {

        fn description(&self) -> &str {
            hook_error_type_as_str(&self.err_type.clone())
        }

        fn cause(&self) -> Option<&Error> {
            self.cause.as_ref().map(|e| &**e)
        }

    }

}

