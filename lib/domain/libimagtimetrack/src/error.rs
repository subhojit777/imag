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
        TimeTrackError, TimeTrackErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
        DateTimeError(::libimagentrydatetime::error::DateError, ::libimagentrydatetime::error::DateErrorKind);
        DatePathError(::libimagentrydatetime::datepath::error::DatePathCompilerError, ::libimagentrydatetime::datepath::error::DatePathCompilerErrorKind);
        TomlError(::toml_query::error::Error, ::toml_query::error::ErrorKind);
    }

    foreign_links {
        ChronoParseError(::chrono::format::ParseError);
    }

    errors {
        HeaderReadError {
            description("Error reading header")
            display("Error reading header")
        }

        TagFormat {
            description("Tag has invalid format")
            display("Tag has invalid format")
        }

        HeaderFieldTypeError {
            description("Type error in header")
            display("Type error in header")
        }
    }
}

