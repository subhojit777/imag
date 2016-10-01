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

use regex::Regex;
use toml::Value;

use libimagstore::store::Entry;

use builtin::header::field_path::FieldPath;
use builtin::header::field_predicate::FieldPredicate;
use builtin::header::field_predicate::Predicate;
use filters::filter::Filter;

struct EqGrep{
    regex: Regex
}

impl Predicate for EqGrep {

    fn evaluate(&self, v: Value) -> bool {
        match v {
            Value::String(s) => self.regex.captures(&s[..]).is_some(),
            _                 => false,
        }
    }

}

/// Check whether certain header field in a entry is equal to a value
pub struct FieldGrep {
    filter: FieldPredicate<EqGrep>,
}

impl FieldGrep {

    pub fn new(path: FieldPath, grep: Regex) -> FieldGrep {
        FieldGrep {
            filter: FieldPredicate::new(path, Box::new(EqGrep { regex: grep})),
        }
    }

}

impl Filter<Entry> for FieldGrep {

    fn filter(&self, e: &Entry) -> bool {
        self.filter.filter(e)
    }

}


