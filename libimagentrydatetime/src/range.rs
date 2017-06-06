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

/// Error types for range module
pub mod error {
    generate_error_module!(
        generate_error_types!(DateTimeRangeError, DateTimeRangeErrorKind,
            EndDateTimeBeforeStartDateTime => "End datetime is before start datetime"
        );
    );

    pub use self::error::DateTimeRangeError;
    pub use self::error::DateTimeRangeErrorKind;
    pub use self::error::MapErrInto;
}

/// Result type for range module
pub mod result {
    use std::result::Result as RResult;
    use super::error::DateTimeRangeError;

    pub type Result<T> = RResult<T, DateTimeRangeError>;
}

use chrono::naive::datetime::NaiveDateTime;
use libimagerror::into::IntoError;
use self::result::Result;

/// A Range between two dates
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTimeRange(NaiveDateTime, NaiveDateTime);

impl DateTimeRange {

    /// Create a new DateTimeRange object
    ///
    /// # Return value
    ///
    /// Ok(DateTimeRange) if start is before end,
    /// else Err(DateTimeRangeError)
    ///
    pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Result<DateTimeRange> {
        use self::error::DateTimeRangeErrorKind as DTREK;
        if start < end {
            Ok(DateTimeRange(start, end))
        } else {
            Err(DTREK::EndDateTimeBeforeStartDateTime.into_error())
        }
    }

}

