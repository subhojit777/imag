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
macro_rules! generate_error_module {
    ( $exprs:item ) => {
        pub mod error {
            $exprs
        }
    }
}

#[macro_export]
macro_rules! generate_error_types {
    {
        $name: ident,
        $kindname: ident,
        $($kind:ident => $string:expr),*
    } => {
        error_chain! {
            types {
                $name, $kindname, ResultExt, Result;
            }

            links {
                // None
            }

            foreign_links {
                // None
            }

            errors {
                $(
                    $kind {
                        description($string)
                        display($string)
                    }
                )*
            }
        }

        generate_result_helper!($name, $kindname);
        generate_option_helper!($name, $kindname);
    }
}

#[macro_export]
macro_rules! generate_result_helper {
    {
        $name:ident, $kindname:ident
    } => {
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
        pub trait MapErrInto<T, E: Error> {
            fn map_err_into(self, error_kind: $kindname) -> ::std::result::Result<T, E>;
        }

        impl<T, E: Error> MapErrInto<T, E: Error> for Result<T, E> {

            fn map_err_into(self, error_kind: $kindname) -> ::std::result::Result<T, E> {
                self.chain_err(|| error_kind)
            }

        }
    }
}

#[macro_export]
macro_rules! generate_option_helper {
    {
        $name:ident, $kindname:ident
    } => {
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
            fn ok_or_errkind(self, kind: $kindname) -> Result<T>;
        }

        impl<T> OkOrErr<T> for Option<T> {

            fn ok_or_errkind(self, kind: $kindname) -> Result<T> {
                self.ok_or($name::from_kind(kind))
            }

        }
    }
}


#[cfg(test)]
#[allow(dead_code)]
mod test {

    generate_error_types!(TestError, TestErrorKind,
        TestErrorKindA => "testerrorkind a",
        TestErrorKindB => "testerrorkind B");

    pub mod anothererrormod {
        generate_error_types!(TestError, TestErrorKind,
            TestErrorKindA => "testerrorkind a",
            TestErrorKindB => "testerrorkind B");
    }

    #[test]
    fn test_error_kind_mapping() {
        use self::MapErrInto;
        use self::TestErrorKind;

        let err : Result<()> = Err(TestError::from_kind(TestErrorKind::TestErrorKindB));
        let err : Result<()> = err.map_err_into(TestErrorKind::TestErrorKindA);

        assert!(err.is_err());

        match *err.unwrap_err().kind() {
            TestErrorKind::TestErrorKindA => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_error_kind_double_mapping() {
        use self::TestErrorKind;
        use std::error::Error;

        let err : Result<()> = Err(TestError::from_kind(TestErrorKind::TestErrorKindB));
        let err : Result<()> = err.map_err_into(TestErrorKind::TestErrorKindA)
                                     .map_err_into(TestErrorKind::TestErrorKindB);

        assert!(err.is_err());
        let err = err.unwrap_err();
        match *err.kind() {
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
        use self::OkOrErr;
        use self::TestErrorKind;

        let something = Some(1);
        match something.ok_or_errkind(TestErrorKind::TestErrorKindA) {
            Ok(1) => assert!(true),
            _     => assert!(false),
        }
    }

    #[test]
    fn test_error_option_bad() {
        use self::OkOrErr;
        use self::TestErrorKind;

        let something : Option<i32> = None;
        match something.ok_or_errkind(TestErrorKind::TestErrorKindA) {
            Ok(_)  => assert!(false),
            Err(_) => assert!(true),
        }
    }

}
