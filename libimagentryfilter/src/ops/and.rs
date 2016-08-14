use libimagstore::store::Entry;

use filters::filter::Filter;

pub struct And {
    a: Box<Filter<Entry>>,
    b: Box<Filter<Entry>>
}

impl And {

    pub fn new(a: Box<Filter<Entry>>, b: Box<Filter<Entry>>) -> And {
        And { a: a, b: b }
    }

}

impl Filter<Entry> for And {

    fn filter(&self, e: &Entry) -> bool {
        self.a.filter(e) && self.b.filter(e)
    }

}
