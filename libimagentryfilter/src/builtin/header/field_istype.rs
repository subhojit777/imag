use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use builtin::header::field_predicate::FieldPredicate;
use builtin::header::field_predicate::Predicate;
use filter::Filter;

use toml::Value;

pub enum Type {
    Array,
    Boolean,
    Float,
    Integer,
    None,
    String,
    Table,
}

impl Type {

    fn matches(&self, v: &Value) -> bool {
        match (self, v) {
            (&Type::String,  &Value::String(_))  => true,
            (&Type::Integer, &Value::Integer(_)) => true,
            (&Type::Float,   &Value::Float(_))   => true,
            (&Type::Boolean, &Value::Boolean(_)) => true,
            (&Type::Array,   &Value::Array(_))   => true,
            (&Type::Table,   &Value::Table(_))   => true,
            _                                    => false,
        }
    }

}

struct IsTypePred {
    ty: Type
}

impl Predicate for IsTypePred {

    fn evaluate(&self, v: Value) -> bool {
        self.ty.matches(&v)
    }

}

pub struct FieldIsType {
    filter: FieldPredicate<IsTypePred>,
}

impl FieldIsType {

    pub fn new(path: FieldPath, expected_type: Type) -> FieldIsType {
        FieldIsType {
            filter: FieldPredicate::new(path, Box::new(IsTypePred { ty: expected_type })),
        }
    }

}

impl Filter for FieldIsType {

    fn filter(&self, e: &Entry) -> bool {
        self.filter.filter(e)
    }

}


