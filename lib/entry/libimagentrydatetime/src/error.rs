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

error_chain! {
    types {
        DateError, DateErrorKind, ResultExt, Result;
    }

    errors {
        DeleteDateError      {
            description("Error deleting date")
            display("Error deleting date")
        }

        ReadDateError        {
            description("Error reading date")
            display("Error reading date")
        }

        SetDateError         {
            description("Error setting date")
            display("Error setting date")
        }

        DeleteDateTimeRangeError {
            description("Error deleting date-time range")
            display("Error deleting date-time range")
        }

        ReadDateTimeRangeError   {
            description("Error reading date-time range")
            display("Error reading date-time range")
        }

        SetDateTimeRangeError    {
            description("Error setting date-time range")
            display("Error setting date-time range")
        }

        DateTimeRangeError  {
            description("DateTime Range error")
            display("DateTime Range error")
        }

        DateHeaderFieldTypeError {
            description("Expected the header field in the entry to have type 'String', but have other type")
            display("Expected the header field in the entry to have type 'String', but have other type")
        }

        DateTimeParsingError {
            description("Error parsing DateTime")
            display("Error parsing DateTime")
        }

        EndDateTimeBeforeStartDateTime {
            description("End datetime is before start datetime")
            display("End datetime is before start datetime")
        }
    }
}

