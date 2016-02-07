use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use filter::Filter;

pub struct ContentLengthIsOver {
    val: usize
}

impl ContentLengthIsOver {

    pub fn new(value: usize) -> ContentLengthIsOver {
        ContentLengthIsOver {
            val: value,
        }
    }

}

impl Filter for ContentLengthIsOver {

    fn filter(&self, e: &Entry) -> bool {
        e.get_content().len() > self.val
    }

}


