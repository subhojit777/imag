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

use filters::filter::Filter;
use libimagstore::store::Entry;

pub struct ContentLengthIsOver {
    val: usize
}

impl ContentLengthIsOver {

    pub fn new(value: usize) -> ContentLengthIsOver {
        ContentLengthIsOver {
            val: value,
        }
    }

}

impl Filter<Entry> for ContentLengthIsOver {

    fn filter(&self, e: &Entry) -> bool {
        e.get_content().len() > self.val
    }

}


