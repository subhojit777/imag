use libimagstore::store::Entry;

use filters::filter::Filter;

pub struct Or {
    a: Box<Filter<Entry>>,
    b: Box<Filter<Entry>>
}

impl Or {

    pub fn new(a: Box<Filter<Entry>>, b: Box<Filter<Entry>>) -> Or {
        Or { a: a, b: b }
    }

}

impl Filter<Entry> for Or {

    fn filter(&self, e: &Entry) -> bool {
        self.a.filter(e) || self.b.filter(e)
    }

}
