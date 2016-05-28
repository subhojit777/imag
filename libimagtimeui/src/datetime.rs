use chrono::naive::datetime::NaiveDateTime as ChronoNaiveDateTime;

use parse::Parse;
use date::Date;
use time::Time;

pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {

    pub fn new(date: Date, time: Time) -> DateTime {
        DateTime {
            date: date,
            time: time
        }
    }

    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

}

impl Into<ChronoNaiveDateTime> for DateTime {

    fn into(self) -> ChronoNaiveDateTime {
        ChronoNaiveDateTime::new(self.date.into(), self.time.into())
    }

}

impl Parse for DateTime {

    fn parse(s: &str) -> Option<DateTime> {
        unimplemented!()
    }

}

