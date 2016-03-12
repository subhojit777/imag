use regex::Regex;
use toml::Value;

use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use builtin::header::field_predicate::FieldPredicate;
use builtin::header::field_predicate::Predicate;
use filter::Filter;

struct EqGrep{
    regex: Regex
}

impl Predicate for EqGrep {

    fn evaluate(&self, v: Value) -> bool {
        match v {
            Value::String(s) => self.regex.captures(&s[..]).is_some(),
            _                 => false,
        }
    }

}

/// Check whether certain header field in a entry is equal to a value
pub struct FieldGrep {
    filter: FieldPredicate<EqGrep>,
}

impl FieldGrep {

    pub fn new(path: FieldPath, grep: Regex) -> FieldGrep {
        FieldGrep {
            filter: FieldPredicate::new(path, Box::new(EqGrep { regex: grep})),
        }
    }

}

impl Filter for FieldGrep {

    fn filter(&self, e: &Entry) -> bool {
        self.filter.filter(e)
    }

}


