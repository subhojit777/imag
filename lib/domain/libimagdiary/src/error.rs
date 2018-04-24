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
        DiaryError, DiaryErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
        EntryUtilError(::libimagentryutil::error::EntryUtilError, ::libimagentryutil::error::EntryUtilErrorKind);
    }

    errors {
        StoreWriteError     {
            description("Error writing store")
            display("Error writing store")
        }

        StoreReadError      {
            description("Error reading store")
            display("Error reading store")
        }

        CannotFindDiary     {
            description("Cannot find diary")
            display("Cannot find diary")
        }

        CannotCreateNote    {
            description("Cannot create Note object for diary entry")
            display("Cannot create Note object for diary entry")
        }

        DiaryEditError      {
            description("Cannot edit diary entry")
            display("Cannot edit diary entry")
        }

        PathConversionError {
            description("Error while converting paths internally")
            display("Error while converting paths internally")
        }

        EntryNotInDiary     {
            description("Entry not in Diary")
            display("Entry not in Diary")
        }

        IOError             {
            description("IO Error")
            display("IO Error")
        }

        ViewError           {
            description("Error viewing diary entry")
            display("Error viewing diary entry")
        }

        IdParseError        {
            description("Error while parsing ID")
            display("Error while parsing ID")
        }

        DiaryNameFindingError {
            description("Error while finding a diary name")
            display("Error while finding a diary name")
        }

    }
}

