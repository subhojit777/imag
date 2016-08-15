use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use builtin::header::field_predicate::FieldPredicate;
use builtin::header::field_predicate::Predicate;
use filters::filter::Filter;

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
            (&Type::String,  &Value::String(_))  |
            (&Type::Integer, &Value::Integer(_)) |
            (&Type::Float,   &Value::Float(_))   |
            (&Type::Boolean, &Value::Boolean(_)) |
            (&Type::Array,   &Value::Array(_))   |
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

impl Filter<Entry> for FieldIsType {

    fn filter(&self, e: &Entry) -> bool {
        self.filter.filter(e)
    }

}


