use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use builtin::header::field_predicate::FieldPredicate;
use builtin::header::field_predicate::Predicate;
use filter::Filter;

use toml::Value;

struct EqGt {
    comp: Value
}

impl Predicate for EqGt {

    fn evaluate(&self, v: Value) -> bool {
        match self.comp {
            Value::Integer(i) => {
                match v {
                    Value::Integer(j) => i > j,
                    Value::Float(f) => (i as f64) > f,
                    _ => false,
                }
            },
            Value::Float(f) => {
                match v {
                    Value::Integer(i) => f > (i as f64),
                    Value::Float(d) => f > d,
                    _ => false,
                }
            },
            _ => false,
        }
    }

}

/// Check whether certain header field in a entry is equal to a value
pub struct FieldGt {
    filter: FieldPredicate<EqGt>,
}

impl FieldGt {

    pub fn new(path: FieldPath, expected_value: Value) -> FieldGt {
        FieldGt {
            filter: FieldPredicate::new(path, Box::new(EqGt { comp: expected_value })),
        }
    }

}

impl Filter for FieldGt {

    fn filter(&self, e: &Entry) -> bool {
        self.filter.filter(e)
    }

}

