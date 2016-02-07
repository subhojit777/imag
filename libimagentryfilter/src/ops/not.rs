use libimagstore::store::Entry;

use filter::Filter;

pub struct Not {
    a: Box<Filter>
}

impl Not {

    pub fn new(a: Box<Filter>) -> Not {
        Not { a: a }
    }

}

impl Filter for Not {

    fn filter(&self, e: &Entry) -> bool {
        !self.a.filter(e)
    }

}
