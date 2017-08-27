//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

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


