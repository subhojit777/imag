//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

#[macro_export]
macro_rules! generate_error_imports {
    () => {
        use std::error::Error;
        use std::fmt::Error as FmtError;
        use std::fmt::{Display, Formatter};

        use $crate::into::IntoError;
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
macro_rules! generate_custom_error_types {
    {
        $name: ident,
        $kindname: ident,
        $customMemberTypeName: ident,
        $($kind:ident => $string:expr),*
    } => {
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
            custom_data: Option<$customMemberTypeName>,
        }

        impl $name {

            pub fn new(errtype: $kindname, cause: Option<Box<Error>>) -> $name {
                $name {
                    err_type: errtype,
                    cause: cause,
                    custom_data: None,
                }
            }

            #[allow(dead_code)]
            pub fn err_type(&self) -> $kindname {
                self.err_type
            }

            #[allow(dead_code)]
            pub fn with_custom_data(mut self, custom: $customMemberTypeName) -> $name {
                self.custom_data = Some(custom);
                self
            }

        }

        impl Into<$name> for $kindname {

            fn into(self) -> $name {
                $name::new(self, None)
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

#[macro_export]
macro_rules! generate_result_helper {
    (
        $name: ident,
        $kindname: ident
    ) => {
        /// Trait to replace
        ///
        /// ```ignore
        /// foo.map_err(Box::new).map_err(|e| SomeType::SomeErrorKind.into_error_with_cause(e))
        /// // or:
        /// foo.map_err(|e| SomeType::SomeErrorKind.into_error_with_cause(Box::new(e)))
        /// ```
        ///
        /// with much nicer
        ///
        /// ```ignore
        /// foo.map_err_into(SomeType::SomeErrorKind)
        /// ```
        ///
        pub trait MapErrInto<T> {
            fn map_err_into(self, error_kind: $kindname) -> Result<T, $name>;
        }

        impl<T, E: Error + 'static> MapErrInto<T> for Result<T, E> {

            fn map_err_into(self, error_kind: $kindname) -> Result<T, $name> {
                self.map_err(Box::new)
                    .map_err(|e| error_kind.into_error_with_cause(e))
            }

        }
    }
}

#[macro_export]
macro_rules! generate_option_helper {
    (
        $name: ident,
        $kindname: ident
    ) => {
        /// Trait to replace
        ///
        /// ```ignore
        /// foo.ok_or(SomeType::SomeErrorKind.into_error())
        /// ```
        ///
        /// with
        ///
        /// ```ignore
        /// foo.ok_or_errkind(SomeType::SomeErrorKind)
        /// ```
        pub trait OkOrErr<T> {
            fn ok_or_errkind(self, kind: $kindname) -> Result<T, $name>;
        }

        impl<T> OkOrErr<T> for Option<T> {

            fn ok_or_errkind(self, kind: $kindname) -> Result<T, $name> {
                self.ok_or(kind.into_error())
            }

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
        #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
        pub struct SomeNotExistingTypeWithATypeNameNoOneWillEverChoose {}
        generate_custom_error_types!($name, $kindname,
                                     SomeNotExistingTypeWithATypeNameNoOneWillEverChoose,
                                     $($kind => $string),*);

        generate_result_helper!($name, $kindname);
        generate_option_helper!($name, $kindname);
    }
}


#[cfg(test)]
mod test {

    generate_error_module!(
        generate_error_types!(TestError, TestErrorKind,
            TestErrorKindA => "testerrorkind a",
            TestErrorKindB => "testerrorkind B");
    );

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
    pub struct CustomData {
        pub test: i32,
        pub othr: i64,
    }

    generate_error_imports!();

    generate_custom_error_types!(CustomTestError, CustomTestErrorKind,
        CustomData,
        CustomErrorKindA => "customerrorkind a",
        CustomErrorKindB => "customerrorkind B");

    impl CustomTestError {
        pub fn test(&self) -> i32 {
            match self.custom_data {
                Some(t) => t.test,
                None => 0,
            }
        }

        pub fn bar(&self) -> i64 {
            match self.custom_data {
                Some(t) => t.othr,
                None => 0,
            }
        }
    }


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

    #[test]
    fn test_error_kind_mapping() {
        use std::io::{Error, ErrorKind};
        use self::error::MapErrInto;
        use self::error::TestErrorKind;

        let err : Result<(), _> = Err(Error::new(ErrorKind::Other, ""));
        let err : Result<(), _> = err.map_err_into(TestErrorKind::TestErrorKindA);

        assert!(err.is_err());
        let err = err.unwrap_err();

        match err.err_type() {
            TestErrorKind::TestErrorKindA => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_error_kind_double_mapping() {
        use std::io::{Error, ErrorKind};
        use self::error::MapErrInto;
        use self::error::TestErrorKind;

        let err : Result<(), _> = Err(Error::new(ErrorKind::Other, ""));
        let err : Result<(), _> = err.map_err_into(TestErrorKind::TestErrorKindA)
                                     .map_err_into(TestErrorKind::TestErrorKindB);

        assert!(err.is_err());
        let err = err.unwrap_err();
        match err.err_type() {
            TestErrorKind::TestErrorKindB => assert!(true),
            _ => assert!(false),
        }

        // not sure how to test that the inner error is of TestErrorKindA, actually...
        match err.cause() {
            Some(_) => assert!(true),
            None    => assert!(false),
        }

    }

    #[test]
    fn test_error_option_good() {
        use self::error::OkOrErr;
        use self::error::TestErrorKind;

        let something = Some(1);
        match something.ok_or_errkind(TestErrorKind::TestErrorKindA) {
            Ok(1) => assert!(true),
            _     => assert!(false),
        }
    }

    #[test]
    fn test_error_option_bad() {
        use self::error::OkOrErr;
        use self::error::TestErrorKind;

        let something : Option<i32> = None;
        match something.ok_or_errkind(TestErrorKind::TestErrorKindA) {
            Ok(_)  => assert!(false),
            Err(e) => assert!(true),
        }
    }

}
