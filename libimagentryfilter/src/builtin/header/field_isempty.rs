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
        let header = e.get_header();
        self.header_field_path
            .walk(header)
            .map(|v| {
                match v {
                    Value::Array(a)   => a.is_empty(),
                    Value::Boolean(_) => false,
                    Value::Float(_)   => false,
                    Value::Integer(_) => false,
                    Value::String(_)  => false,
                    Value::Table(t)   => t.is_empty(),
                    _                 => true,
                }
            })
            .unwrap_or(false)
    }

}



