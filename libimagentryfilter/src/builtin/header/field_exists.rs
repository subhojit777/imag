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
use filters::filter::Filter;

pub struct FieldExists {
    header_field_path: FieldPath,
}

impl FieldExists {

    pub fn new(path: FieldPath) -> FieldExists {
        FieldExists {
            header_field_path: path,
        }
    }

}

impl Filter<Entry> for FieldExists {

    fn filter(&self, e: &Entry) -> bool {
        e.get_header().read(&self.header_field_path[..]).is_ok()
    }

}


