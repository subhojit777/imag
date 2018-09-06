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

use toml_query::read::TomlValueReadExt;
use toml::Value;

use builtin::header::field_path::FieldPath;
use filters::failable::filter::FailableFilter;
use error::Result;
use error::FilterError as FE;

pub trait Predicate {
    fn evaluate(&self, &Value) -> bool;
}

/// Check whether certain header field in a entry is equal to a value
///
/// # Notice
///
/// Returns false if the value is not present.
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

impl<P: Predicate> FailableFilter<Entry> for FieldPredicate<P> {
    type Error = FE;

    fn filter(&self, e: &Entry) -> Result<bool> {
        Ok(e.get_header()
            .read(&self.header_field_path[..])?
            .map(|v| (*self.predicate).evaluate(v))
            .unwrap_or(false)
            )
    }

}

