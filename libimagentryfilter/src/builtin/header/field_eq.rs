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
        e.get_header()
            .read(&self.header_field_path[..])
            .map(|val| val.map(|v| v == self.expected_value).unwrap_or(false))
            .unwrap_or(false)
    }

}

