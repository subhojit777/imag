use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use filter::Filter;

use toml::Value;

pub struct FieldIsEmpty {
    header_field_path: FieldPath,
}

impl FieldIsEmpty {

    pub fn new(path: FieldPath) -> FieldIsEmpty {
        FieldIsEmpty {
            header_field_path: path,
        }
    }

}

impl Filter for FieldIsEmpty {

    fn filter(&self, e: &Entry) -> bool {
        e.get_header()
            .read(&self.header_field_path[..])
            .map(|v| {
                match v {
                    Some(Value::Array(a))   => a.is_empty(),
                    Some(Value::Boolean(_)) => false,
                    Some(Value::Float(_))   => false,
                    Some(Value::Integer(_)) => false,
                    Some(Value::String(s))  => s.is_empty(),
                    Some(Value::Table(t))   => t.is_empty(),
                    _                       => true,
                }
            })
            .unwrap_or(false)
    }

}



