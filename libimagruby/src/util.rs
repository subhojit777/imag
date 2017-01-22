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

use ruru::AnyObject;
pub trait Wrap {
    fn wrap(self) -> AnyObject;
}

#[macro_export]
macro_rules! impl_wrap {
    ($target:ty => $wrapper:path) => {
        impl Wrap for $target {
            fn wrap(self) -> AnyObject {
                Class::from_existing(concat!("R", stringify!($target)))
                    .wrap_data(self, &*($wrapper))
            }
        }
    }
}

pub trait Unwrap {
    type Target;
    fn unwrap<'a>(&'a self) -> &'a mut Self::Target;
}

#[macro_export]
macro_rules! impl_unwrap {
    ($from:ty => $to:ty => $wrapper:path) => {
        impl Unwrap for $from {
            type Target = $to;
            fn unwrap<'a>(&'a self) -> &'a mut $to {
                self.get_data(&*($wrapper))
            }
        }
    }
}

#[macro_export]
macro_rules! impl_verified_object {
    ($objname: ty) => {
        impl VerifiedObject for $objname {
            fn is_correct_type<T: Object>(object: &T) -> bool {
                object.class() == Class::from_existing(stringify!($objname))
            }

            fn error_message() -> &'static str {
                concat!("Not a ", stringify!($objname), " object")
            }
        }
    };
}

/// Helper macro to simplify type checking in the ruby-interfacing functions.
///
/// # Return
///
/// If called with only the object to check, this returns NIL after raising an exception.
/// If called with more arguments, the other things will be returned.
/// E.G.:
///
/// ```ignore
/// let obj1 = typecheck!(obj1); // returns `obj` or raises exception
///
/// // returns `obj` or raises exception and returns AnyObject (Boolean -> false):
/// let obj2 = typecheck!(obj2 or return any Boolean::new(false));
///
/// // returns `obj` or raises excpetion and returns Boolean -> false
/// let obj3 = typecheck!(obj3 or return Boolean::new(false));
/// ```
///
#[macro_export]
macro_rules! typecheck {
    ($obj: ident)                          => { typecheck!($obj or return NilClass::new()) };
    ($obj: ident or return any $els: expr) => { typecheck!($obj or return $els.to_any_object()) };
    ($obj: ident or return $els: expr)     => {
        match $obj {
            Ok(o)  => o,
            Err(e) => {
                VM::raise(e.to_exception(), e.description());
                return $els
            },
        }
    };

}

