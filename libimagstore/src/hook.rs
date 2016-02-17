use std::fmt::Debug;

use toml::Value;

use self::error::HookError;

pub type HookResult<T> = Result<T, HookError>;

pub trait Configureable {
    fn set_config(&mut self, cfg: Value);
}

pub trait Hook : Configureable + Debug {
}

pub mod read {
    use storeid::StoreId;
    use store::FileLockEntry;
    use super::HookResult;
    use super::Hook;

    pub trait PreReadHook : Hook {
        fn pre_read(&self, &StoreId) -> HookResult<()>;
    }

    pub trait PostReadHook : Hook {
        fn post_read<'a>(&self, FileLockEntry<'a>) -> HookResult<FileLockEntry<'a>>;
    }

}

pub mod create {
    use storeid::StoreId;
    use store::FileLockEntry;
    use super::HookResult;
    use super::Hook;

    pub trait PreCreateHook : Hook {
        fn pre_create(&self, &StoreId) -> HookResult<()>;
    }

    pub trait PostCreateHook : Hook {
        fn post_create<'a>(&self, FileLockEntry<'a>) -> HookResult<FileLockEntry<'a>>;
    }

}

pub mod retrieve {
    use storeid::StoreId;
    use store::FileLockEntry;
    use super::HookResult;
    use super::Hook;

    pub trait PreRetrieveHook : Hook {
        fn pre_retrieve(&self, &StoreId) -> HookResult<()>;
    }

    pub trait PostRetrieveHook : Hook {
        fn post_retrieve<'a>(&self, FileLockEntry<'a>) -> HookResult<FileLockEntry<'a>>;
    }
}

pub mod update {
    use store::FileLockEntry;
    use super::HookResult;
    use super::Hook;

    pub trait PreUpdateHook : Hook {
        fn pre_update(&self, &FileLockEntry) -> HookResult<()>;
    }

    pub trait PostUpdateHook : Hook {
        fn post_update(&self, &FileLockEntry) -> HookResult<()>;
    }
}

pub mod delete {
    use storeid::StoreId;
    use store::FileLockEntry;
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

    /**
     * Kind of error
     */
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum HookErrorKind {
        // ...
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

