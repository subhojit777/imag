use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use filter::Filter;

use toml::Value;

pub enum Type {
    Array,
    Boolean,
    Float,
    Integer,
    None,
    String,
    Table,
}

impl Type {

    fn matches(&self, v: &Value) -> bool {
        match (self, v) {
            (&Type::String,  &Value::String(_))  => true,
            (&Type::Integer, &Value::Integer(_)) => true,
            (&Type::Float,   &Value::Float(_))   => true,
            (&Type::Boolean, &Value::Boolean(_)) => true,
            (&Type::Array,   &Value::Array(_))   => true,
            (&Type::Table,   &Value::Table(_))   => true,
            _                                    => false,
        }
    }

}

pub struct FieldIsType {
    header_field_path: FieldPath,
    expected_type: Type,
}

impl FieldIsType {

    pub fn new(path: FieldPath, expected_type: Type) -> FieldIsType {
        FieldIsType {
            header_field_path: path,
            expected_type: expected_type,
        }
    }

}

impl Filter for FieldIsType {

    fn filter(&self, e: &Entry) -> bool {
        let header = e.get_header();
        self.header_field_path
            .walk(header)
            .map(|v| self.expected_type.matches(&v))
            .unwrap_or(false)
    }

}


