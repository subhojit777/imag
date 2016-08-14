use libimagstore::store::Entry;

use filters::filter::Filter;

pub struct BoolFilter(bool);

impl BoolFilter {

    pub fn new(b: bool) -> BoolFilter {
        BoolFilter(b)
    }

}

impl Filter<Entry> for BoolFilter {

    fn filter(&self, _: &Entry) -> bool {
        self.0
    }

}

