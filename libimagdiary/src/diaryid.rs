use std::convert::Into;
use std::str::FromStr;

use chrono::naive::datetime::NaiveDateTime;
use chrono::naive::time::NaiveTime;
use chrono::naive::date::NaiveDate;
use chrono::Datelike;
use chrono::Timelike;
use regex::Regex;

use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;

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

}

impl IntoStoreId for DiaryId {

    fn into_storeid(self) -> StoreId {
        let s : String = self.into();
        ModuleEntryPath::new(s).into_storeid()
    }

}

impl Into<String> for DiaryId {

    fn into(self) -> String {
        format!("{}/{}/{}-{}-{}:{}",
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

impl FromStoreId for DiaryId {

    fn from_storeid(s: &StoreId) -> Option<DiaryId> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
            (.*)
            /(?P<name>(.*))
            /(?P<year>\d{4})
            /(?P<month>\d{2})
            -(?P<day>\d{2})
            -(?P<hour>\d{2})
            :(?P<minute>\d{2})
            "
            ).unwrap();
        }

        s.to_str()
            .map(|s| { debug!("StoreId = {:?}", s); s })
            .and_then(|s| RE.captures(s))
            .and_then(|caps| {
                let name   = caps.at(0);
                let year   = caps.at(1);
                let month  = caps.at(2);
                let day    = caps.at(3);
                let hour   = caps.at(4);
                let minute = caps.at(5);

                debug!("some? name   = {:?}", name.is_some());
                debug!("some? year   = {:?}", year.is_some());
                debug!("some? month  = {:?}", month.is_some());
                debug!("some? day    = {:?}", day.is_some());
                debug!("some? hour   = {:?}", hour.is_some());
                debug!("some? minute = {:?}", minute.is_some());

                if [name, year, month, day, hour, minute].iter().all(|x| x.is_some()) {
                    let year = {
                        match i32::from_str(year.unwrap()) {
                            Ok(x) => x,
                            Err(_) => return None,
                        }
                    };

                    let month = {
                        match u32::from_str(month.unwrap()) {
                            Ok(x) => x,
                            Err(_) => return None,
                        }
                    };

                    let day = {
                        match u32::from_str(day.unwrap()) {
                            Ok(x) => x,
                            Err(_) => return None,
                        }
                    };

                    let hour = {
                        match u32::from_str(hour.unwrap()) {
                            Ok(x) => x,
                            Err(_) => return None,
                        }
                    };

                    let minute = {
                        match u32::from_str(minute.unwrap()) {
                            Ok(x) => x,
                            Err(_) => return None,
                        }
                    };

                    Some(DiaryId {
                        name   : String::from(name.unwrap()),
                        year   : year,
                        month  : month,
                        day    : day,
                        hour   : hour,
                        minute : minute,
                    })
                } else {
                    None
                }
            })
    }

}

