use filters::filter::Filter;
use libimagstore::store::Entry;

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

impl Filter<Entry> for ContentLengthIsUnder {

    fn filter(&self, e: &Entry) -> bool {
        e.get_content().len() < self.val
    }

}


