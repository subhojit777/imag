use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use filter::Filter;

use toml::Value;

pub struct FieldExists {
    header_field_path: FieldPath,
}

impl FieldExists {

    pub fn new(path: FieldPath) -> FieldExists {
        FieldExists {
            header_field_path: path,
        }
    }

}

impl Filter for FieldExists {

    fn filter(&self, e: &Entry) -> bool {
        let header = e.get_header();
        self.header_field_path.walk(header).is_some()
    }

}


