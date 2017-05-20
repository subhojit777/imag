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

use chrono::naive::datetime::NaiveDateTime;
use toml_query::delete::TomlValueDeleteExt;
use toml_query::insert::TomlValueInsertExt;
use toml_query::read::TomlValueReadExt;
use toml::Value;

use libimagstore::store::Entry;
use libimagerror::into::IntoError;

use error::DateErrorKind as DEK;
use error::*;
use result::Result;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateRange(NaiveDateTime, NaiveDateTime);

pub trait EntryDate {

    fn delete_date(&mut self) -> Result<()>;
    fn read_date(&self) -> Result<NaiveDateTime>;
    fn set_date(&mut self, d: NaiveDateTime) -> Result<Option<Result<NaiveDateTime>>>;

    fn delete_date_range(&mut self) -> Result<()>;
    fn read_date_range(&self) -> Result<DateRange>;
    fn set_date_range(&mut self, start: NaiveDateTime, end: NaiveDateTime) -> Result<Option<Result<DateRange>>>;

}

lazy_static! {
    static ref DATE_HEADER_LOCATION : String             = String::from("date.value");
    static ref DATE_RANGE_START_HEADER_LOCATION : String = String::from("date.range.start");
    static ref DATE_RANGE_END_HEADER_LOCATION : String   = String::from("date.range.end");
    static ref DATE_FMT : &'static str                   = "%Y-%m-%d %H:%M:%S";
}

impl EntryDate for Entry {

    fn delete_date(&mut self) -> Result<()> {
        self.get_header_mut()
            .delete(&DATE_HEADER_LOCATION)
            .map(|_| ())
            .map_err_into(DEK::DeleteDateError)
    }

    fn read_date(&self) -> Result<NaiveDateTime> {
        self.get_header()
            .read(&DATE_HEADER_LOCATION)
            .map_err_into(DEK::ReadDateError)
            .and_then(|v| {
                match v {
                    &Value::String(ref s) => s.parse::<NaiveDateTime>()
                        .map_err_into(DEK::DateTimeParsingError),
                    _ => Err(DEK::DateHeaderFieldTypeError.into_error()),
                }
            })
    }

    /// Set a Date for this entry
    ///
    /// # Return value
    ///
    /// This function returns funny things, I know. But I find it more attractive to be explicit
    /// what failed when here, instead of beeing nice to the user here.
    ///
    /// So here's a list how things are returned:
    ///
    /// - Err(_) if the inserting failed
    /// - Ok(None) if the inserting succeeded and _did not replace an existing value_.
    /// - Ok(Some(Ok(_))) if the inserting succeeded, but replaced an existing value which then got
    /// parsed into a NaiveDateTime object
    /// - Ok(Some(Err(_))) if the inserting succeeded, but replaced an existing value which then
    /// got parsed into a NaiveDateTime object, where the parsing failed for some reason.
    ///
    fn set_date(&mut self, d: NaiveDateTime) -> Result<Option<Result<NaiveDateTime>>> {
        let date = d.format(&DATE_FMT).to_string();

        self.get_header_mut()
            .insert(&DATE_HEADER_LOCATION, Value::String(date))
            .map(|opt| opt.map(|stri| {
                match stri {
                    Value::String(ref s) => s.parse::<NaiveDateTime>()
                                             .map_err_into(DEK::DateTimeParsingError),
                    _ => Err(DEK::DateHeaderFieldTypeError.into_error()),
                }
            }))
            .map_err_into(DEK::SetDateError)
    }


    /// Deletes the date range
    ///
    /// # Warning
    ///
    /// First deletes the start, then the end. If the first operation fails, this might leave the
    /// header in an inconsistent state.
    ///
    fn delete_date_range(&mut self) -> Result<()> {
        let _ = try!(self
             .get_header_mut()
            .delete(&DATE_RANGE_START_HEADER_LOCATION)
            .map(|_| ())
            .map_err_into(DEK::DeleteDateRangeError));

        self.get_header_mut()
            .delete(&DATE_RANGE_END_HEADER_LOCATION)
            .map(|_| ())
            .map_err_into(DEK::DeleteDateRangeError)
    }

    fn read_date_range(&self) -> Result<DateRange> {
        let start = try!(self
            .get_header()
            .read(&DATE_RANGE_START_HEADER_LOCATION)
            .map_err_into(DEK::ReadDateRangeError)
            .and_then(|v| {
                match v {
                    &Value::String(ref s) => s.parse::<NaiveDateTime>()
                        .map_err_into(DEK::DateTimeParsingError),
                    _ => Err(DEK::DateHeaderFieldTypeError.into_error()),
                }
            }));

        let end = try!(self
            .get_header()
            .read(&DATE_RANGE_START_HEADER_LOCATION)
            .map_err_into(DEK::ReadDateRangeError)
            .and_then(|v| {
                match v {
                    &Value::String(ref s) => s.parse::<NaiveDateTime>()
                        .map_err_into(DEK::DateTimeParsingError),
                    _ => Err(DEK::DateHeaderFieldTypeError.into_error()),
                }
            }));

        Ok(DateRange(start, end))
    }

    /// Set the date range
    ///
    /// # Warning
    ///
    /// This first sets the start, then the end. If the first operation fails, this might leave the
    /// header in an inconsistent state.
    ///
    fn set_date_range(&mut self, start: NaiveDateTime, end: NaiveDateTime)
        -> Result<Option<Result<DateRange>>>
    {
        let start = start.format(&DATE_FMT).to_string();
        let end   = end.format(&DATE_FMT).to_string();

        let opt_old_start = try!(self
            .get_header_mut()
            .insert(&DATE_RANGE_START_HEADER_LOCATION, Value::String(start))
            .map(|opt| opt.map(|stri| {
                match stri {
                    Value::String(ref s) => s.parse::<NaiveDateTime>()
                                             .map_err_into(DEK::DateTimeParsingError),
                    _ => Err(DEK::DateHeaderFieldTypeError.into_error()),
                }
            }))
            .map_err_into(DEK::SetDateRangeError));

        let opt_old_end = try!(self
            .get_header_mut()
            .insert(&DATE_RANGE_END_HEADER_LOCATION, Value::String(end))
            .map(|opt| opt.map(|stri| {
                match stri {
                    Value::String(ref s) => s.parse::<NaiveDateTime>()
                                             .map_err_into(DEK::DateTimeParsingError),
                    _ => Err(DEK::DateHeaderFieldTypeError.into_error()),
                }
            }))
            .map_err_into(DEK::SetDateRangeError));

        match (opt_old_start, opt_old_end) {
            (Some(Ok(old_start)), Some(Ok(old_end))) => Ok(Some(Ok(DateRange(old_start, old_end)))),

            (Some(Err(e)), _) => Err(e),
            (_, Some(Err(e))) => Err(e),
            _ => {
                Ok(None)
            },
        }
    }

}

