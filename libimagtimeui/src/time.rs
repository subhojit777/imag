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

use chrono::naive::time::NaiveTime as ChronoNaiveTime;

use parse::Parse;

pub struct Time {
    hour:   u32,
    minute: u32,
    second: u32,
}

impl Time {

    pub fn new(hour: u32, minute: u32, second: u32) -> Time {
        Time {
            hour: hour,
            minute: minute,
            second: second
        }
    }

    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn second(&self) -> u32 {
        self.second
    }

}

impl Into<ChronoNaiveTime> for Time {

    fn into(self) -> ChronoNaiveTime {
        ChronoNaiveTime::from_hms(self.hour, self.minute, self.second)
    }

}

impl Parse for Time {

    fn parse(s: &str) -> Option<Time> {
        use std::str::FromStr;
        use regex::Regex;
        use parse::time_parse_regex;

        lazy_static! {
            static ref R: Regex = Regex::new(time_parse_regex()).unwrap();
        }

        R.captures(s)
            .and_then(|capts| {
                let minute = capts.name("m").and_then(|o| FromStr::from_str(o).ok()).unwrap_or(0);
                let second = capts.name("s").and_then(|o| FromStr::from_str(o).ok()).unwrap_or(0);
                let hour   = match capts.name("h").and_then(|o| FromStr::from_str(o).ok()) {
                    None => {
                        debug!("No hour");
                        return None;
                    },
                    Some(x) => x,
                };

                Some(Time::new(hour, minute, second))
            })
    }

}

#[cfg(test)]
mod test {
    use super::Time;
    use parse::Parse;

    #[test]
    fn test_valid() {
        let s = "2016-12-12T20:01:02";
        let t = Time::parse(s);

        assert!(t.is_some());
        let t = t.unwrap();

        assert_eq!(20, t.hour());
        assert_eq!(1, t.minute());
        assert_eq!(2, t.second());
    }

    #[test]
    fn test_valid_without_sec() {
        let s = "2016-12-12T20:01";
        let t = Time::parse(s);

        assert!(t.is_some());
        let t = t.unwrap();

        assert_eq!(20, t.hour());
        assert_eq!(1, t.minute());
        assert_eq!(0, t.second());
    }

    #[test]
    fn test_valid_without_min() {
        let s = "2016-12-12T20";
        let t = Time::parse(s);

        assert!(t.is_some());
        let t = t.unwrap();

        assert_eq!(20, t.hour());
        assert_eq!(0, t.minute());
        assert_eq!(0, t.second());
    }

    #[test]
    fn test_invalid() {
        assert!(Time::parse("2015-12-12T").is_none());
        assert!(Time::parse("2015-12-12T200").is_none());
        assert!(Time::parse("2015-12-12T20-20").is_none());
        assert!(Time::parse("2015-12-12T20:200").is_none());
        assert!(Time::parse("2015-12-12T20:20:200").is_none());
        assert!(Time::parse("2015-12-12T20:20:").is_none());
        assert!(Time::parse("2015-12-12T20:").is_none());
        assert!(Time::parse("2015-12-12T2:20:21").is_none());
        assert!(Time::parse("2015-12-12T2:2:20").is_none());
        assert!(Time::parse("2015-12-12T2:2:2").is_none());
    }

}

