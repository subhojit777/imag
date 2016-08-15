use filters::filter::Filter;
use libimagstore::store::Entry;

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

impl Filter<Entry> for ContentLengthIsOver {

    fn filter(&self, e: &Entry) -> bool {
        e.get_content().len() > self.val
    }

}


