use std::convert::Into;

use chrono::naive::datetime::NaiveDateTime;
use chrono::naive::time::NaiveTime;
use chrono::naive::date::NaiveDate;
use chrono::Datelike;
use chrono::Timelike;

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
        format!("{}/{}/{}/{}/{}:{}",
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

        let day    = if day.is_none()    { return None; } else { day.unwrap() };
        let month  = if month.is_none()  { return None; } else { month.unwrap() };
        let year   = if year.is_none()   { return None; } else { year.unwrap() };
        let name   = if name.is_none()   { return None; } else { name.unwrap() };

        Some(DiaryId::new(name, year, month, day, hour, minute))
    }

}

