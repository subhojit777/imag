use regex::Regex;
use toml::Value;

use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use filter::Filter;

/// Check whether certain header field in a entry is equal to a value
pub struct FieldGrep {
    header_field_path: FieldPath,
    grep: Regex,
}

impl FieldGrep {

    pub fn new(path: FieldPath, grep: Regex) -> FieldGrep {
        FieldGrep {
            header_field_path: path,
            grep: grep,
        }
    }

}

impl Filter for FieldGrep {

    fn filter(&self, e: &Entry) -> bool {
        e.get_header()
            .read(&self.header_field_path[..])
            .map(|v| {
                match v {
                    Some(Value::String(s)) => self.grep.captures(&s[..]).is_some(),
                    _                      => false,
                }
            })
            .unwrap_or(false)
    }

}


