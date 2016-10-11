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

use std::convert::Into;
use std::fmt::{Display, Formatter, Error as FmtError};

use chrono::naive::datetime::NaiveDateTime;
use chrono::naive::time::NaiveTime;
use chrono::naive::date::NaiveDate;
use chrono::Datelike;
use chrono::Timelike;

use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagstore::store::Result as StoreResult;

use module_path::ModuleEntryPath;

#[derive(Debug, Clone)]
pub struct DiaryId {
    name: String,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl DiaryId {

    pub fn new(name: String, y: i32, m: u32, d: u32, h: u32, min: u32) -> DiaryId {
        DiaryId {
            name: name,
            year: y,
            month: m,
            day: d,
            hour: h,
            minute: min,
        }
    }

    pub fn from_datetime<DT: Datelike + Timelike>(diary_name: String, dt: DT) -> DiaryId {
        DiaryId::new(diary_name, dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute())
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

    pub fn now(name: String) -> DiaryId {
        use chrono::offset::local::Local;

        let now = Local::now();
        let now_date = now.date().naive_local();
        let now_time = now.time();
        let dt = NaiveDateTime::new(now_date, now_time);

        DiaryId::new(name, dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute())
    }

}

impl Default for DiaryId {

    /// Create a default DiaryId which is a diaryid for a diary named "default" with
    /// time = 0000-00-00 00:00:00
    fn default() -> DiaryId {
        let dt = NaiveDateTime::new(NaiveDate::from_ymd(0, 0, 0), NaiveTime::from_hms(0, 0, 0));
        DiaryId::from_datetime(String::from("default"), dt)
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
        format!("{}/{:0>4}/{:0>2}/{:0>2}/{:0>2}:{:0>2}",
                self.name, self.year, self.month, self.day, self.hour, self.minute)
    }

}

impl Display for DiaryId {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        write!(fmt, "{}/{:0>4}/{:0>2}/{:0>2}/{:0>2}:{:0>2}",
                self.name, self.year, self.month, self.day, self.hour, self.minute)
    }

}

impl Into<NaiveDateTime> for DiaryId {

    fn into(self) -> NaiveDateTime {
        let d = NaiveDate::from_ymd(self.year, self.month, self.day);
        let t = NaiveTime::from_hms(self.hour, self.minute, 0);
        NaiveDateTime::new(d, t)
    }

}

pub trait FromStoreId : Sized {

    fn from_storeid(&StoreId) -> Option<Self>;

}

use std::path::Component;

fn component_to_str<'a>(com: Component<'a>) -> Option<&'a str> {
    match com {
        Component::Normal(s) => Some(s),
        _ => None
    }.and_then(|s| s.to_str())
}

impl FromStoreId for DiaryId {

    fn from_storeid(s: &StoreId) -> Option<DiaryId> {
        use std::str::FromStr;

        let mut cmps   = s.components().rev();
        let (hour, minute) = match cmps.next().and_then(component_to_str)
            .and_then(|time| {
                let mut time = time.split(":");
                let hour     = time.next().and_then(|s| FromStr::from_str(s).ok());
                let minute   = time.next()
                    .and_then(|s| s.split("~").next())
                    .and_then(|s| FromStr::from_str(s).ok());

                debug!("Hour   = {:?}", hour);
                debug!("Minute = {:?}", minute);

                match (hour, minute) {
                    (Some(h), Some(m)) => Some((h, m)),
                    _ => None,
                }
            })
        {
            Some(s) => s,
            None => return None,
        };

        let day   :Option<u32> = cmps.next().and_then(component_to_str).and_then(|s| FromStr::from_str(s).ok());
        let month :Option<u32> = cmps.next().and_then(component_to_str).and_then(|s| FromStr::from_str(s).ok());
        let year  :Option<i32> = cmps.next().and_then(component_to_str).and_then(|s| FromStr::from_str(s).ok());
        let name       = cmps.next().and_then(component_to_str).map(String::from);

        debug!("Day   = {:?}", day);
        debug!("Month = {:?}", month);
        debug!("Year  = {:?}", year);
        debug!("Name  = {:?}", name);

        let day    = match day   { None => return None, Some(day)   => day };
        let month  = match month { None => return None, Some(month) => month };
        let year   = match year  { None => return None, Some(year)  => year };
        let name   = match name  { None => return None, Some(name)  => name };

        Some(DiaryId::new(name, year, month, day, hour, minute))
    }

}

