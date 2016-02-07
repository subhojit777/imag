use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use filter::Filter;

use toml::Value;

/// Check whether certain header field in a entry is equal to a value
pub struct FieldEq {
    header_field_path: FieldPath,
    expected_value: Value
}

impl FieldEq {

    pub fn new(path: FieldPath, expected_value: Value) -> FieldEq {
        FieldEq {
            header_field_path: path,
            expected_value: expected_value,
        }
    }

}

impl Filter for FieldEq {

    fn filter(&self, e: &Entry) -> bool {
        let header = e.get_header();
        self.header_field_path
            .walk(header)
            .map(|v| self.expected_value == v.clone())
            .unwrap_or(false)
    }

}

