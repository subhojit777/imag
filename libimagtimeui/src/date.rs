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

use chrono::naive::date::NaiveDate as ChronoNaiveDate;

use parse::Parse;

pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl Date {

    pub fn new(year: i32, month: u32, day: u32) -> Date {
        Date {
            year: year,
            month: month,
            day: day,
        }
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn day(&self) -> u32 {
        self.day
    }

}

impl Into<ChronoNaiveDate> for Date {

    fn into(self) -> ChronoNaiveDate {
        ChronoNaiveDate::from_ymd(self.year, self.month, self.day)
    }

}

impl Parse for Date {

    /// Parse the date part of the full string into a Date object
    fn parse(s: &str) -> Option<Date> {
        use std::str::FromStr;
        use regex::Regex;
        use parse::time_parse_regex;

        lazy_static! {
            static ref R: Regex = Regex::new(time_parse_regex()).unwrap();
        }

        R.captures(s)
            .and_then(|capts| {
                let year  = capts.name("Y").and_then(|o| FromStr::from_str(o).ok());
                let month = capts.name("M").and_then(|o| FromStr::from_str(o).ok());
                let day   = capts.name("D").and_then(|o| FromStr::from_str(o).ok());

                let year = match year {
                    None => {
                        debug!("No year");
                        return None;
                    },
                    Some(x) => x,
                };

                let month = match month {
                    None => {
                        debug!("No month");
                        return None;
                    },
                    Some(x) => x,
                };

                let day = match day {
                    None => {
                        debug!("No day");
                        return None;
                    },
                    Some(x) => x,
                };


                Some(Date::new(year, month, day))
            })

    }

}

#[cfg(test)]
mod test {
    use super::Date;
    use parse::Parse;

    #[test]
    fn test_valid() {
        let s = "2016-02-01";
        let d = Date::parse(s);

        assert!(d.is_some());
        let d = d.unwrap();

        assert_eq!(2016, d.year());
        assert_eq!(2, d.month());
        assert_eq!(1, d.day());
    }

    #[test]
    fn test_invalid() {
        assert!(Date::parse("2016-021-01").is_none());
        assert!(Date::parse("2016-02-012").is_none());
        assert!(Date::parse("2016-02-0").is_none());
        assert!(Date::parse("2016-0-02").is_none());
        assert!(Date::parse("2016-02").is_none());
        assert!(Date::parse("2016-2").is_none());
    }

}

