use libimagstore::store::Entry;

use filter::Filter;

pub struct BoolFilter(bool);

impl BoolFilter {

    pub fn new(b: bool) -> BoolFilter {
        BoolFilter(b)
    }

}

impl Filter for BoolFilter {

    fn filter(&self, _: &Entry) -> bool {
        self.0
    }

}

