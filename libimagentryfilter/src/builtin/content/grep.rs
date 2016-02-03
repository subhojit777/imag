use std::convert::Into;

use regex::Regex;
use regex::Error as RError;

use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use filter::Filter;

pub trait IntoRegex {

    fn into_regex(self) -> Result<Regex, RError>;

}

impl<'a> IntoRegex for &'a str {

    fn into_regex(self) -> Result<Regex, RError> {
        Regex::new(self)
    }
}

impl<'a> IntoRegex for Regex {

    fn into_regex(self) -> Result<Regex, RError> {
        Ok(self)
    }
}

pub struct ContentGrep {
    regex: Regex,
}

impl ContentGrep {

    pub fn new<IR>(regex: IR) -> Result<ContentGrep, RError>
        where IR: IntoRegex
    {
        regex.into_regex()
            .map(|reg| {
                ContentGrep {
                    regex: reg,
                }
            })
    }

}

impl Filter for ContentGrep {

    fn filter(&self, e: &Entry) -> bool {
        self.regex.captures(&e.get_content()[..]).is_some()
    }

}

