use chrono::naive::time::NaiveTime as ChronoNaiveTime;

use parse::Parse;

pub struct Time {
    hour:   u32,
    minute: u32,
    second: u32,
}

impl Time {

    fn new(hour: u32, minute: u32, second: u32) -> Time {
        unimplemented!()
    }

}

impl Into<ChronoNaiveTime> for Time {

    fn into(self) -> ChronoNaiveTime {
        ChronoNaiveTime::from_hms(self.hour, self.minute, self.second)
    }

}

impl Parse for Time {

    fn parse(s: &str) -> Option<Time> {
        unimplemented!()
    }

}

