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

    errors {
        StoreReadError {
            description("Error while writing Store")
            display("Error while writing Store")
        }

        StoreWriteError {
            description("Error while reading Store")
            display("Error while reading Store")
        }

        StoreIdError {
            description("Error while handling StoreId")
            display("Error while handling StoreId")
        }

        TagFormat {
            description("Tag has invalid format")
            display("Tag has invalid format")
        }

        HeaderReadError {
            description("Error writing header")
            display("Error writing header")
        }
        HeaderWriteError {
            description("Error writing header")
            display("Error writing header")
        }
        HeaderFieldTypeError {
            description("Type error in header")
            display("Type error in header")
        }
        DateTimeParserError {
            description("Error while parsing DateTime")
            display("Error while parsing DateTime")
        }
    }
}

