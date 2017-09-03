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
        RefError, RefErrorKind, ResultExt, Result;
    }

    links {
        ListError(::libimagentrylist::error::ListError, ::libimagentrylist::error::ListErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        StoreReadError          {
            description("Store read error")
            display("Store read error")
        }

        StoreWriteError         {
            description("Store write error")
            display("Store write error")
        }

        IOError                 {
            description("IO Error")
            display("IO Error")
        }

        UTF8Error               {
            description("UTF8 Error")
            display("UTF8 Error")
        }

        StoreIdError            {
            description("Error with storeid")
            display("Error with storeid")
        }

        HeaderTomlError         {
            description("Error while working with TOML Header")
            display("Error while working with TOML Header")
        }

        HeaderTypeError         {
            description("Header type error")
            display("Header type error")
        }

        HeaderFieldMissingError {
            description("Header field missing error")
            display("Header field missing error")
        }

        HeaderFieldWriteError   {
            description("Header field cannot be written")
            display("Header field cannot be written")
        }

        HeaderFieldReadError    {
            description("Header field cannot be read")
            display("Header field cannot be read")
        }

        HeaderFieldAlreadyExistsError {
            description("Header field already exists, cannot override")
            display("Header field already exists, cannot override")
        }

        PathUTF8Error {
            description("Path cannot be converted because of UTF8 Error")
            display("Path cannot be converted because of UTF8 Error")
        }

        PathHashingError {
            description("Path cannot be hashed")
            display("Path cannot be hashed")
        }

        PathCanonicalizationError {
            description("Path cannot be canonicalized")
            display("Path cannot be canonicalized")
        }

        TypeConversionError {
            description("Couldn't convert types")
            display("Couldn't convert types")
        }

        RefToDisplayError {
            description("Cannot convert Ref to string to show it to user")
            display("Cannot convert Ref to string to show it to user")
        }

        RefNotInStore {
            description("Ref/StoreId does not exist in store")
            display("Ref/StoreId does not exist in store")
        }

        RefTargetDoesNotExist       {
            description("Ref Target does not exist")
            display("Ref Target does not exist")
        }

        RefTargetPermissionError    {
            description("Ref Target permissions insufficient for referencing")
            display("Ref Target permissions insufficient for referencing")
        }

        RefTargetCannotBeHashed     {
            description("Ref Target cannot be hashed (is it a directory?)")
            display("Ref Target cannot be hashed (is it a directory?)")
        }

        RefTargetFileCannotBeOpened {
            description("Ref Target File cannot be open()ed")
            display("Ref Target File cannot be open()ed")
        }

        RefTargetCannotReadPermissions {
            description("Ref Target: Cannot read permissions")
            display("Ref Target: Cannot read permissions")
        }

        RefHashingError {
            description("Error while hashing")
            display("Error while hashing")
        }

    }
}

impl IntoError for RefErrorKind {
    type Target = RefError;

    fn into_error(self) -> Self::Target {
        RefError::from_kind(self)
    }

    fn into_error_with_cause(self, _: Box<Error>) -> Self::Target {
        RefError::from_kind(self)
    }
}
