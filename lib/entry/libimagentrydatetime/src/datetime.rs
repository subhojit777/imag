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

use chrono::naive::NaiveDateTime;
use toml_query::delete::TomlValueDeleteExt;
use toml_query::insert::TomlValueInsertExt;
use toml_query::read::TomlValueReadExt;
use toml::Value;

use libimagstore::store::Entry;
use libimagerror::into::IntoError;

use error::DateErrorKind as DEK;
use error::*;
use result::Result;
use range::DateTimeRange;

pub trait EntryDate {

    fn delete_date(&mut self) -> Result<()>;
    fn read_date(&self) -> Result<NaiveDateTime>;
    fn set_date(&mut self, d: NaiveDateTime) -> Result<Option<Result<NaiveDateTime>>>;

    fn delete_date_range(&mut self) -> Result<()>;
    fn read_date_range(&self) -> Result<DateTimeRange>;
    fn set_date_range(&mut self, start: NaiveDateTime, end: NaiveDateTime) -> Result<Option<Result<DateTimeRange>>>;

}

lazy_static! {
    static ref DATE_HEADER_LOCATION : &'static str              = "datetime.value";
    static ref DATE_RANGE_START_HEADER_LOCATION : &'static str  = "datetime.range.start";
    static ref DATE_RANGE_END_HEADER_LOCATION : &'static str    = "datetime.range.end";
    static ref DATE_FMT : &'static str                          = "%Y-%m-%dT%H:%M:%S";
}

impl EntryDate for Entry {

    fn delete_date(&mut self) -> Result<()> {
        self.get_header_mut()
            .delete(&DATE_HEADER_LOCATION)
            .map(|_| ())
            .chain_err(|| DEK::DeleteDateError)
    }

    fn read_date(&self) -> Result<NaiveDateTime> {
        self.get_header()
            .read(&DATE_HEADER_LOCATION)
            .chain_err(|| DEK::ReadDateError)
            .and_then(|v| {
                match v {
                    Some(&Value::String(ref s)) => s.parse::<NaiveDateTime>()
                        .chain_err(|| DEK::DateTimeParsingError),
                    Some(_) => Err(DEK::DateHeaderFieldTypeError.into_error()),
                    _ => Err(DEK::ReadDateError.into_error()),
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
                                             .chain_err(|| DEK::DateTimeParsingError),
                    _ => Err(DEK::DateHeaderFieldTypeError.into_error()),
                }
            }))
            .chain_err(|| DEK::SetDateError)
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
            .chain_err(|| DEK::DeleteDateTimeRangeError));

        self.get_header_mut()
            .delete(&DATE_RANGE_END_HEADER_LOCATION)
            .map(|_| ())
            .chain_err(|| DEK::DeleteDateTimeRangeError)
    }

    fn read_date_range(&self) -> Result<DateTimeRange> {
        let start = try!(self
            .get_header()
            .read(&DATE_RANGE_START_HEADER_LOCATION)
            .chain_err(|| DEK::ReadDateTimeRangeError)
            .and_then(|v| {
                match v {
                    Some(&Value::String(ref s)) => s.parse::<NaiveDateTime>()
                        .chain_err(|| DEK::DateTimeParsingError),
                    Some(_) => Err(DEK::DateHeaderFieldTypeError.into_error()),
                    _ => Err(DEK::ReadDateError.into_error()),
                }
            }));

        let end = try!(self
            .get_header()
            .read(&DATE_RANGE_START_HEADER_LOCATION)
            .chain_err(|| DEK::ReadDateTimeRangeError)
            .and_then(|v| {
                match v {
                    Some(&Value::String(ref s)) => s.parse::<NaiveDateTime>()
                        .chain_err(|| DEK::DateTimeParsingError),
                    Some(_) => Err(DEK::DateHeaderFieldTypeError.into_error()),
                    _ => Err(DEK::ReadDateError.into_error()),
                }
            }));

        DateTimeRange::new(start, end)
            .chain_err(|| DEK::DateTimeRangeError)
    }

    /// Set the date range
    ///
    /// # Warning
    ///
    /// This first sets the start, then the end. If the first operation fails, this might leave the
    /// header in an inconsistent state.
    ///
    fn set_date_range(&mut self, start: NaiveDateTime, end: NaiveDateTime)
        -> Result<Option<Result<DateTimeRange>>>
    {
        let start = start.format(&DATE_FMT).to_string();
        let end   = end.format(&DATE_FMT).to_string();

        let opt_old_start = try!(self
            .get_header_mut()
            .insert(&DATE_RANGE_START_HEADER_LOCATION, Value::String(start))
            .map(|opt| opt.map(|stri| {
                match stri {
                    Value::String(ref s) => s.parse::<NaiveDateTime>()
                                             .chain_err(|| DEK::DateTimeParsingError),
                    _ => Err(DEK::DateHeaderFieldTypeError.into_error()),
                }
            }))
            .chain_err(|| DEK::SetDateTimeRangeError));

        let opt_old_end = try!(self
            .get_header_mut()
            .insert(&DATE_RANGE_END_HEADER_LOCATION, Value::String(end))
            .map(|opt| opt.map(|stri| {
                match stri {
                    Value::String(ref s) => s.parse::<NaiveDateTime>()
                                             .chain_err(|| DEK::DateTimeParsingError),
                    _ => Err(DEK::DateHeaderFieldTypeError.into_error()),
                }
            }))
            .chain_err(|| DEK::SetDateTimeRangeError));

        match (opt_old_start, opt_old_end) {
            (Some(Ok(old_start)), Some(Ok(old_end))) => {
                let dr = DateTimeRange::new(old_start, old_end)
                    .chain_err(|| DEK::DateTimeRangeError);

                Ok(Some(dr))
            },

            (Some(Err(e)), _) => Err(e),
            (_, Some(Err(e))) => Err(e),
            _ => {
                Ok(None)
            },
        }
    }

}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    use libimagstore::store::Store;

    use chrono::naive::NaiveDateTime;
    use chrono::naive::NaiveDate;
    use chrono::naive::NaiveTime;

    pub fn get_store() -> Store {
        Store::new(PathBuf::from("/"), None).unwrap()
    }

    #[test]
    fn test_set_date() {
        let store = get_store();

        let date = {
            let date = NaiveDate::from_ymd(2000, 01, 02);
            let time = NaiveTime::from_hms(03, 04, 05);

            NaiveDateTime::new(date, time)
        };

        let mut entry   = store.create(PathBuf::from("test")).unwrap();
        let res         = entry.set_date(date);

        assert!(res.is_ok(), format!("Error: {:?}", res));
        let res = res.unwrap();

        assert!(res.is_none()); // There shouldn't be an existing value

        // Check whether the header is set correctly

        let hdr_field = entry.get_header().read(&DATE_HEADER_LOCATION);

        assert!(hdr_field.is_ok());
        let hdr_field = hdr_field.unwrap();

        assert!(hdr_field.is_some());
        let hdr_field = hdr_field.unwrap();

        match *hdr_field {
            Value::String(ref s) => assert_eq!("2000-01-02T03:04:05", s),
            _ => assert!(false, "Wrong header type"),
        }
    }

    #[test]
    fn test_read_date() {
        use chrono::Datelike;
        use chrono::Timelike;

        let store = get_store();

        let date = {
            let date = NaiveDate::from_ymd(2000, 01, 02);
            let time = NaiveTime::from_hms(03, 04, 05);

            NaiveDateTime::new(date, time)
        };

        let mut entry   = store.create(PathBuf::from("test")).unwrap();
        let res         = entry.set_date(date);

        assert!(res.is_ok(), format!("Expected Ok(_), got: {:?}", res));
        let res = res.unwrap();

        assert!(res.is_none()); // There shouldn't be an existing value

        // same as the test above ...

        let d = entry.read_date();

        assert!(d.is_ok(), format!("Expected Ok(_), got: {:?}", d));
        let d = d.unwrap();

        assert_eq!(d.date().year()   , 2000);
        assert_eq!(d.date().month()  ,   01);
        assert_eq!(d.date().day()    ,   02);
        assert_eq!(d.time().hour()   ,   03);
        assert_eq!(d.time().minute() ,   04);
        assert_eq!(d.time().second() ,   05);
    }

    #[test]
    fn test_delete_date() {
        let store = get_store();

        let date = {
            let date = NaiveDate::from_ymd(2000, 01, 02);
            let time = NaiveTime::from_hms(03, 04, 05);

            NaiveDateTime::new(date, time)
        };

        let mut entry   = store.create(PathBuf::from("test")).unwrap();
        let res         = entry.set_date(date);

        assert!(res.is_ok(), format!("Expected Ok(_), got: {:?}", res));
        let res = res.unwrap();
        assert!(res.is_none()); // There shouldn't be an existing value

        assert!(entry.delete_date().is_ok());

        let hdr_field = entry.get_header().read(&DATE_HEADER_LOCATION);

        assert!(hdr_field.is_ok());
        let hdr_field = hdr_field.unwrap();

        assert!(hdr_field.is_none());
    }
}

