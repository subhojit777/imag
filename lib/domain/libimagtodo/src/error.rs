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
        TodoError, TodoErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
    }

    foreign_links {
        TomlQueryError(::toml_query::error::Error);
    }

    errors {
        ConversionError     {
            description("Conversion Error")
            display("Conversion Error")
        }

        StoreIdError        {
            description("Store Id handling error")
            display("Store Id handling error")
        }

        ImportError         {
            description("Error importing")
            display("Error importing")
        }

        UTF8Error           {
            description("Encountered non-UTF8 characters while reading input")
            display("Encountered non-UTF8 characters while reading input")
        }

        HeaderFieldMissing {
            description("Header field missing")
            display("Header field missing")
        }

        HeaderTypeError {
            description("Header field type error")
            display("Header field type error")
        }

        UuidParserError {
            description("Uuid parser error")
            display("Uuid parser error")
        }
    }
}

