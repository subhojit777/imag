//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

error_chain! {
    types {
        RefError, RefErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
        TomlQueryError(::toml_query::error::Error, ::toml_query::error::ErrorKind);
        EntryUtilError(::libimagentryutil::error::EntryUtilError, ::libimagentryutil::error::EntryUtilErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Utf8Error(::std::string::FromUtf8Error);
        TomlDeError(::toml::de::Error);
        TomlSerError(::toml::ser::Error);
    }

    errors {
        HeaderTypeError(field: &'static str, expectedtype: &'static str) {
            description("Header type error")
            display("Header type error: '{}' should be {}", field, expectedtype)
        }

        HeaderFieldMissingError(field: &'static str) {
            description("Header field missing error")
            display("Header field missing: {}", field)
        }

        HeaderFieldWriteError {
            description("Header field cannot be written")
            display("Header field cannot be written")
        }

        HeaderFieldReadError {
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

    }
}

