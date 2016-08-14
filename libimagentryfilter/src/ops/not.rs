use libimagstore::store::Entry;

use filters::filter::Filter;

pub struct Not {
    a: Box<Filter<Entry>>
}

impl Not {

    pub fn new(a: Box<Filter<Entry>>) -> Not {
        Not { a: a }
    }

}

impl Filter<Entry> for Not {

    fn filter(&self, e: &Entry) -> bool {
        !self.a.filter(e)
    }

}
