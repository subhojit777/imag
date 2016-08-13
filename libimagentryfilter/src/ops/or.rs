use libimagstore::store::Entry;

use filters::filter::Filter;

pub struct Or {
    a: Box<Filter>,
    b: Box<Filter>
}

impl Or {

    pub fn new(a: Box<Filter>, b: Box<Filter>) -> Or {
        Or { a: a, b: b }
    }

}

impl Filter for Or {

    fn filter(&self, e: &Entry) -> bool {
        self.a.filter(e) || self.b.filter(e)
    }

}
