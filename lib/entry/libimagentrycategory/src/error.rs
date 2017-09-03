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

use std::error::Error;

use libimagerror::into::IntoError;

error_chain! {
    types {
        CategoryError, CategoryErrorKind, ResultExt, Result;
    }

    errors {
        StoreReadError {
            description("Store Read error")
            display("Store Read error")
        }

        StoreWriteError {
            description("Store Write error")
            display("Store Write error")
        }

        StoreIdHandlingError {
            description("StoreId handling error")
            display("StoreId handling error")
        }

        HeaderReadError  {
            description("Header read error")
            display("Header read error")
        }

        HeaderWriteError {
            description("Header write error")
            display("Header write error")
        }

        CategoryDoesNotExist {
            description("Category does not exist")
            display("Category does not exist")
        }

        TypeError {
            description("Type Error")
            display("Type Error")
        }
    }
}

impl IntoError for CategoryErrorKind {
    type Target = CategoryError;

    fn into_error(self) -> Self::Target {
        CategoryError::from_kind(self)
    }

    fn into_error_with_cause(self, _: Box<Error>) -> Self::Target {
        CategoryError::from_kind(self)
    }
}
