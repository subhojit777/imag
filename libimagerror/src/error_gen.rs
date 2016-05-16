use into::IntoError;

#[macro_export]
macro_rules! generate_error_imports {
    () => {
        use std::error::Error;
        use std::fmt::Error as FmtError;
        use std::fmt::{Display, Formatter};
    }
}

#[macro_export]
macro_rules! generate_error_module {
    ( $exprs:item ) => {
        pub mod error {
            generate_error_imports!();
            $exprs
        }
    }
}

#[macro_export]
macro_rules! generate_error_types {
    (
        $name: ident,
        $kindname: ident,
        $($kind:ident => $string:expr),*
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $kindname {
            $( $kind ),*
        }

        impl Display for $kindname {

            fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
                let s = match *self {
                    $( $kindname::$kind => $string ),*
                };
                try!(write!(fmt, "{}", s));
                Ok(())
            }

        }

        impl IntoError for $kindname {
            type Target = $name;

            fn into_error(self) -> Self::Target {
                $name::new(self, None)
            }

            fn into_error_with_cause(self, cause: Box<Error>) -> Self::Target {
                $name::new(self, Some(cause))
            }

        }

        #[derive(Debug)]
        pub struct $name {
            err_type: $kindname,
            cause: Option<Box<Error>>,
        }

        impl $name {

            pub fn new(errtype: $kindname, cause: Option<Box<Error>>) -> $name {
                $name {
                    err_type: errtype,
                    cause: cause,
                }
            }

            pub fn err_type(&self) -> $kindname {
                self.err_type
            }

        }

        impl Display for $name {

            fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
                try!(write!(fmt, "[{}]", self.err_type));
                Ok(())
            }

        }

        impl Error for $name {

            fn description(&self) -> &str {
                match self.err_type {
                    $( $kindname::$kind => $string ),*
                }
            }

            fn cause(&self) -> Option<&Error> {
                self.cause.as_ref().map(|e| &**e)
            }

        }

    }
}

#[cfg(test)]
mod test {

    generate_error_module!(
        generate_error_types!(TestError, TestErrorKind,
            TestErrorKindA => "testerrorkind a",
            TestErrorKindB => "testerrorkind B");
    );

    #[test]
    fn test_a() {
        use self::error::{TestError, TestErrorKind};

        let kind = TestErrorKind::TestErrorKindA;
        assert_eq!(String::from("testerrorkind a"), format!("{}", kind));

        let e = TestError::new(kind, None);
        assert_eq!(String::from("[testerrorkind a]"), format!("{}", e));
    }

    #[test]
    fn test_b() {
        use self::error::{TestError, TestErrorKind};

        let kind = TestErrorKind::TestErrorKindB;
        assert_eq!(String::from("testerrorkind B"), format!("{}", kind));

        let e = TestError::new(kind, None);
        assert_eq!(String::from("[testerrorkind B]"), format!("{}", e));

    }

    #[test]
    fn test_ab() {
        use std::error::Error;
        use self::error::{TestError, TestErrorKind};

        let kinda = TestErrorKind::TestErrorKindA;
        let kindb = TestErrorKind::TestErrorKindB;
        assert_eq!(String::from("testerrorkind a"), format!("{}", kinda));
        assert_eq!(String::from("testerrorkind B"), format!("{}", kindb));

        let e = TestError::new(kinda, Some(Box::new(TestError::new(kindb, None))));
        assert_eq!(String::from("[testerrorkind a]"), format!("{}", e));
        assert_eq!(TestErrorKind::TestErrorKindA, e.err_type());
        assert_eq!(String::from("[testerrorkind B]"), format!("{}", e.cause().unwrap()));
    }

    pub mod anothererrormod {
        generate_error_imports!();
        generate_error_types!(TestError, TestErrorKind,
            TestErrorKindA => "testerrorkind a",
            TestErrorKindB => "testerrorkind B");
    }

    #[test]
    fn test_other_a() {
        use self::anothererrormod::{TestError, TestErrorKind};

        let kind = TestErrorKind::TestErrorKindA;
        assert_eq!(String::from("testerrorkind a"), format!("{}", kind));

        let e = TestError::new(kind, None);
        assert_eq!(String::from("[testerrorkind a]"), format!("{}", e));
    }

    #[test]
    fn test_other_b() {
        use self::anothererrormod::{TestError, TestErrorKind};

        let kind = TestErrorKind::TestErrorKindB;
        assert_eq!(String::from("testerrorkind B"), format!("{}", kind));

        let e = TestError::new(kind, None);
        assert_eq!(String::from("[testerrorkind B]"), format!("{}", e));

    }

    #[test]
    fn test_other_ab() {
        use std::error::Error;
        use self::anothererrormod::{TestError, TestErrorKind};

        let kinda = TestErrorKind::TestErrorKindA;
        let kindb = TestErrorKind::TestErrorKindB;
        assert_eq!(String::from("testerrorkind a"), format!("{}", kinda));
        assert_eq!(String::from("testerrorkind B"), format!("{}", kindb));

        let e = TestError::new(kinda, Some(Box::new(TestError::new(kindb, None))));
        assert_eq!(String::from("[testerrorkind a]"), format!("{}", e));
        assert_eq!(TestErrorKind::TestErrorKindA, e.err_type());
        assert_eq!(String::from("[testerrorkind B]"), format!("{}", e.cause().unwrap()));
    }
}
