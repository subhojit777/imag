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

use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIterator;

use notestoreid::*;
use error::Result;
use error::NoteError as NE;

#[derive(Debug)]
pub struct NoteIterator(StoreIdIterator);

impl NoteIterator {

    pub fn new(iditer: StoreIdIterator) -> NoteIterator {
        NoteIterator(iditer)
    }

}

impl Iterator for NoteIterator {
    type Item = Result<StoreId>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(n) = self.0.next() {
            match n {
                Ok(n) => if n.is_note_id() {
                    return Some(Ok(n));
                },
                Err(e) => return Some(Err(e).map_err(NE::from)),
            }
        }

        None
    }

}

