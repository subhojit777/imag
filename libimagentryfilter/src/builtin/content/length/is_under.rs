use libimagstore::store::Entry;

use filter::Filter;

pub struct ContentLengthIsUnder {
    val: usize
}

impl ContentLengthIsUnder {

    pub fn new(value: usize) -> ContentLengthIsUnder {
        ContentLengthIsUnder {
            val: value,
        }
    }

}

impl Filter for ContentLengthIsUnder {

    fn filter(&self, e: &Entry) -> bool {
        e.get_content().len() < self.val
    }

}


