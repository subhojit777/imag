use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use builtin::header::field_predicate::FieldPredicate;
use builtin::header::field_predicate::Predicate;
use filters::filter::Filter;

use toml::Value;

struct EqPred {
    expected: Value
}

impl Predicate for EqPred {

    fn evaluate(&self, v: Value) -> bool {
        self.expected == v
    }

}

/// Check whether certain header field in a entry is equal to a value
pub struct FieldEq {
    filter: FieldPredicate<EqPred>,
}

impl FieldEq {

    pub fn new(path: FieldPath, expected_value: Value) -> FieldEq {
        FieldEq {
            filter: FieldPredicate::new(path, Box::new(EqPred { expected: expected_value })),
        }
    }

}

impl Filter<Entry> for FieldEq {

    fn filter(&self, e: &Entry) -> bool {
        self.filter.filter(e)
    }

}

