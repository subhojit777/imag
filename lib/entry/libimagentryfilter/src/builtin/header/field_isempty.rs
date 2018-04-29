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

use builtin::header::field_path::FieldPath;
use filters::failable::filter::FailableFilter;
use error::Result;
use error::FilterError as FE;

use toml::Value;

pub struct FieldIsEmpty {
    header_field_path: FieldPath,
}

impl FieldIsEmpty {

    pub fn new(path: FieldPath) -> FieldIsEmpty {
        FieldIsEmpty {
            header_field_path: path,
        }
    }

}

impl FailableFilter<Entry> for FieldIsEmpty {
    type Error = FE;

    fn filter(&self, e: &Entry) -> Result<bool> {
        Ok(e
             .get_header()
             .read(&self.header_field_path[..])?
             .map(|v| {
                 match v {
                     &Value::Array(ref a)   => a.is_empty(),
                     &Value::String(ref s)  => s.is_empty(),
                     &Value::Table(ref t)   => t.is_empty(),
                     &Value::Boolean(_)  |
                     &Value::Float(_)    |
                     &Value::Datetime(_) |
                     &Value::Integer(_)  => false,
                 }
             })
             .unwrap_or(true))
    }

}



