use storeid::StoreId;
use store::FileLockEntry;

use self::error::HookError;

pub type HookResult = Result<(), HookError>;

pub trait Hook {
    fn description(&self) -> &str;
    fn dependencies(&self) -> Vec<Box<Hook>>;
}

pub trait StoreIdHook : Hook {
    fn execute(&self, &StoreId) -> HookResult;
}

pub trait FileLockHook : Hook {
    fn execute(&self, &StoreId, &FileLockEntry) -> HookResult;
}

pub mod read {
    use super::FileLockHook;
    use super::StoreIdHook;

    pub trait PreReadHook : StoreIdHook {
    }

    pub trait PostReadHook : FileLockHook {
    }

}

pub mod create {
    use super::FileLockHook;
    use super::StoreIdHook;

    pub trait PreCreateHook : StoreIdHook {
    }

    pub trait PostCreateHook : FileLockHook {
    }

}

pub mod retrieve {
    use super::FileLockHook;
    use super::StoreIdHook;

    pub trait PreRetrieveHook : StoreIdHook {
    }

    pub trait PostRetrieveHook : FileLockHook {
    }
}

pub mod delete {
    use super::StoreIdHook;

    pub trait PreDeleteHook : StoreIdHook {
    }

    pub trait PostDeleteHook : StoreIdHook {
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

