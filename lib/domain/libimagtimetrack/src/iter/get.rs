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

use libimagstore::storeid::StoreIdIteratorWithStore;
use libimagstore::store::Store;
use libimagstore::store::Result as StoreResult;
use libimagstore::store::FileLockEntry;

use constants::*;

pub struct TimeTrackingsGetIterator<'a>(StoreIdIteratorWithStore<'a>, &'a Store);

impl<'a> TimeTrackingsGetIterator<'a> {
    pub fn new(sit: StoreIdIteratorWithStore<'a>, store: &'a Store) -> Self {
        TimeTrackingsGetIterator(sit, store)
    }
}

impl<'a> Iterator for TimeTrackingsGetIterator<'a> {
    type Item = StoreResult<FileLockEntry<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            match next {
                Err(e)   => return Some(Err(e)),
                Ok(next) => if next.is_in_collection(&[CRATE_NAME]) {
                    return match self.1.get(next) {
                        Ok(Some(fle)) => Some(Ok(fle)),
                        Ok(None)      => continue,
                        Err(e)        => Some(Err(e))
                    };
                }
            }
        }

        None
    }

}

