use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use filter::Filter;

use toml::Value;

pub trait Predicate {
    fn evaluate(&self, Value) -> bool;
}

/// Check whether certain header field in a entry is equal to a value
pub struct FieldPredicate<P: Predicate> {
    header_field_path: FieldPath,
    predicate: Box<P>,
}

impl<P: Predicate> FieldPredicate<P> {

    pub fn new(path: FieldPath, predicate: Box<P>) -> FieldPredicate<P> {
        FieldPredicate {
            header_field_path: path,
            predicate: predicate,
        }
    }

}

impl<P: Predicate> Filter for FieldPredicate<P> {

    fn filter(&self, e: &Entry) -> bool {
        e.get_header()
            .read(&self.header_field_path[..])
            .map(|val| {
                match val {
                    None => false,
                    Some(v) => (*self.predicate).evaluate(v),
                }
            })
            .unwrap_or(false)
    }

}


