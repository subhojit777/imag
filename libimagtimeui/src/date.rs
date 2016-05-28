use chrono::naive::date::NaiveDate as ChronoNaiveDate;

use parse::Parse;

pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl Date {

    fn new(year: i32, month: u32, day: u32) -> Date {
        unimplemented!()
    }

}

impl Into<ChronoNaiveDate> for Date {

    fn into(self) -> ChronoNaiveDate {
        ChronoNaiveDate::from_ymd(self.year, self.month, self.day)
    }

}

impl Parse for Date {

    fn parse(s: &str) -> Option<Date> {
        unimplemented!()
    }

}

