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

use error::TimeTrackError as TTE;
use error::TimeTrackErrorKind as TTEK;
use error::ResultExt;

use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagstore::storeid::StoreIdIterator;

pub struct GetTimeTrackIter<'a>{
    inner: StoreIdIterator,
    store: &'a Store,
}

impl<'a> GetTimeTrackIter<'a> {

    pub fn new(sidit: StoreIdIterator, store: &'a Store) -> GetTimeTrackIter<'a> {
        GetTimeTrackIter {
            inner: sidit,
            store: store
        }
    }
}

impl<'a> Iterator for GetTimeTrackIter<'a> {
    type Item = Result<FileLockEntry<'a>, TTE>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|sid| {
            self.store
                .get(sid)
                .chain_err(|| TTEK::StoreReadError)?
                .ok_or(TTE::from_kind(TTEK::StoreReadError))
        })
    }
}

// impl<'a, I> From<I> for GetTimeTrackIter<'a, I>
//     where I: Iterator<Item = Result<FileLockEntry<'a>, TTE>>
// {
//     fn from(i: I) -> GetTimeTrackIter<'a, I> {
//         GetTimeTrackIter(i)
//     }
// }
//
