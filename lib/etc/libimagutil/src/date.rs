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

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::format::ParseError;

pub const NAIVE_DATE_STRING_FORMAT : &'static str = "%Y-%m-%d";

pub fn date_to_string(ndt: &NaiveDate) -> String {
    ndt.format(NAIVE_DATE_STRING_FORMAT).to_string()
}

pub fn date_from_string(s: String) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(&s, NAIVE_DATE_STRING_FORMAT)
}

pub const NAIVE_DATETIME_STRING_FORMAT : &'static str = "%Y-%m-%d %H:%M:%S";

pub fn datetime_to_string(ndt: &NaiveDateTime) -> String {
    ndt.format(NAIVE_DATETIME_STRING_FORMAT).to_string()
}

pub fn datetime_from_string(s: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(s, NAIVE_DATETIME_STRING_FORMAT)
}

