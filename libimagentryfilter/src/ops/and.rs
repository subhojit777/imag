use libimagstore::store::Entry;

use filter::Filter;

pub struct And {
    a: Box<Filter>,
    b: Box<Filter>
}

impl And {

    pub fn new(a: Box<Filter>, b: Box<Filter>) -> And {
        And { a: a, b: b }
    }

}

impl Filter for And {

    fn filter(&self, e: &Entry) -> bool {
        self.a.filter(e) && self.b.filter(e)
    }

}
