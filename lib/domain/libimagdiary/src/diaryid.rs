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

use std::convert::Into;
use std::fmt::{Display, Formatter, Error as FmtError};

use chrono::naive::NaiveDateTime;
use chrono::naive::NaiveTime;
use chrono::naive::NaiveDate;
use chrono::Datelike;
use chrono::Timelike;

use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagstore::store::Result as StoreResult;

use error::DiaryError as DE;
use error::DiaryErrorKind as DEK;
use error::ResultExt;

use module_path::ModuleEntryPath;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct DiaryId {
    name: String,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl DiaryId {

    pub fn new(name: String, y: i32, m: u32, d: u32, h: u32, min: u32, sec: u32) -> DiaryId {
        DiaryId {
            name: name,
            year: y,
            month: m,
            day: d,
            hour: h,
            minute: min,
            second: sec,
        }
    }

    pub fn from_datetime<DT: Datelike + Timelike>(diary_name: String, dt: DT) -> DiaryId {
        DiaryId::new(diary_name,
                     dt.year(),
                     dt.month(),
                     dt.day(),
                     dt.hour(),
                     dt.minute(),
                     dt.second())
    }

    pub fn diary_name(&self) -> &String {
        &self.name
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

    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn second(&self) -> u32 {
        self.second
    }

    pub fn with_diary_name(mut self, name: String) -> DiaryId {
        self.name = name;
        self
    }

    pub fn with_year(mut self, year: i32) -> DiaryId {
        self.year = year;
        self
    }

    pub fn with_month(mut self, month: u32) -> DiaryId {
        self.month = month;
        self
    }

    pub fn with_day(mut self, day: u32) -> DiaryId {
        self.day = day;
        self
    }

    pub fn with_hour(mut self, hour: u32) -> DiaryId {
        self.hour = hour;
        self
    }

    pub fn with_minute(mut self, minute: u32) -> DiaryId {
        self.minute = minute;
        self
    }

    pub fn with_second(mut self, sec: u32) -> DiaryId {
        self.second = sec;
        self
    }

    pub fn now(name: String) -> DiaryId {
        use chrono::offset::Local;

        let now = Local::now();
        let now_date = now.date().naive_local();
        let now_time = now.time();
        let dt = NaiveDateTime::new(now_date, now_time);

        DiaryId::new(name, dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute(), dt.second())
    }

}

impl IntoStoreId for DiaryId {

    fn into_storeid(self) -> StoreResult<StoreId> {
        let s : String = self.into();
        ModuleEntryPath::new(s).into_storeid()
    }

}

impl Into<String> for DiaryId {

    fn into(self) -> String {
        format!("{}/{:0>4}/{:0>2}/{:0>2}/{:0>2}:{:0>2}:{:0>2}",
                self.name, self.year, self.month, self.day, self.hour, self.minute, self.second)
    }

}

impl Display for DiaryId {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        write!(fmt, "{}/{:0>4}/{:0>2}/{:0>2}/{:0>2}:{:0>2}:{:0>2}",
                self.name, self.year, self.month, self.day, self.hour, self.minute, self.second)
    }

}

impl Into<NaiveDateTime> for DiaryId {

    fn into(self) -> NaiveDateTime {
        let d = NaiveDate::from_ymd(self.year, self.month, self.day);
        let t = NaiveTime::from_hms(self.hour, self.minute, self.second);
        NaiveDateTime::new(d, t)
    }

}

pub trait FromStoreId : Sized {

    fn from_storeid(&StoreId) -> Result<Self, DE>;

}

use std::path::Component;

fn component_to_str<'a>(com: Component<'a>) -> Result<&'a str, DE> {
    match com {
        Component::Normal(s) => Some(s),
        _ => None,
    }.and_then(|s| s.to_str())
    .ok_or(DE::from_kind(DEK::IdParseError))
}

impl FromStoreId for DiaryId {

    fn from_storeid(s: &StoreId) -> Result<DiaryId, DE> {
        use std::str::FromStr;

        use std::path::Components;
        use std::iter::Rev;

        fn next_component<'a>(components: &'a mut Rev<Components>) -> Result<&'a str, DE> {
            components.next()
                .ok_or(DE::from_kind(DEK::IdParseError))
                .and_then(component_to_str)
        }

        let mut cmps   = s.components().rev();

        let (hour, minute, second) = next_component(&mut cmps).and_then(|time| {
            let mut time = time.split(":");
            let hour     = time.next().and_then(|s| FromStr::from_str(s).ok());
            let minute   = time.next().and_then(|s| FromStr::from_str(s).ok());
            let second   = time.next().and_then(|s| FromStr::from_str(s).ok());

            debug!("Hour   = {:?}", hour);
            debug!("Minute = {:?}", minute);
            debug!("Second = {:?}", second);

            match (hour, minute, second) {
                (Some(h), Some(m), Some(s)) => Ok((h, m, s)),
                _ => return Err(DE::from_kind(DEK::IdParseError)),
            }
        })?;

        let day: Result<u32,_> = next_component(&mut cmps)
            .and_then(|s| s.parse::<u32>()
                      .chain_err(|| DEK::IdParseError));

        let month: Result<u32,_> = next_component(&mut cmps)
            .and_then(|s| s.parse::<u32>()
                      .chain_err(|| DEK::IdParseError));

        let year: Result<i32,_> = next_component(&mut cmps)
            .and_then(|s| s.parse::<i32>()
                      .chain_err(|| DEK::IdParseError));

        let name = next_component(&mut cmps).map(String::from);

        debug!("Day   = {:?}", day);
        debug!("Month = {:?}", month);
        debug!("Year  = {:?}", year);
        debug!("Name  = {:?}", name);

        let day    = day?;
        let month  = month?;
        let year   = year?;
        let name   = name?;

        Ok(DiaryId::new(name, year, month, day, hour, minute, second))
    }

}

