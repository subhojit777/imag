use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use filter::Filter;

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
        e.get_header().read(&self.header_field_path[..]).is_ok()
    }

}


