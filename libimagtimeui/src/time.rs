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
        use std::str::FromStr;
        use regex::Regex;
        use parse::time_parse_regex;

        lazy_static! {
            static ref R: Regex = Regex::new(time_parse_regex()).unwrap();
        }

        R.captures(s)
            .and_then(|capts| {
                let hour   = capts.name("h").and_then(|o| FromStr::from_str(o).ok());
                let minute = capts.name("m").and_then(|o| FromStr::from_str(o).ok()).unwrap_or(0);
                let second = capts.name("s").and_then(|o| FromStr::from_str(o).ok()).unwrap_or(0);

                if hour.is_none() {
                    debug!("No hour");
                    return None;
                }

                Some(Time::new(hour.unwrap(), minute, second))
            })
    }

}

