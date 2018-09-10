//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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
use filters::failable::filter::FailableFilter;
use error::Result;
use error::FilterError as FE;

use toml::Value;

struct EqLt {
    comp: Value
}

impl Predicate for EqLt {

    fn evaluate(&self, v: &Value) -> bool {
        match self.comp {
            Value::Integer(i) => {
                match *v {
                    Value::Integer(j) => i < j,
                    Value::Float(f) => (i as f64) < f,
                    _ => false,
                }
            },
            Value::Float(f) => {
                match *v {
                    Value::Integer(i) => f < (i as f64),
                    Value::Float(d) => f < d,
                    _ => false,
                }
            },
            _ => false,
        }
    }

}

/// Check whether certain header field in a entry is equal to a value
pub struct FieldLt {
    filter: FieldPredicate<EqLt>,
}

impl FieldLt {

    pub fn new(path: FieldPath, expected_value: Value) -> FieldLt {
        FieldLt {
            filter: FieldPredicate::new(path, Box::new(EqLt { comp: expected_value })),
        }
    }

}

impl FailableFilter<Entry> for FieldLt {
    type Error = FE;

    fn filter(&self, e: &Entry) -> Result<bool> {
        self.filter.filter(e)
    }

}

