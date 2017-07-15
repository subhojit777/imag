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

use error::TimeTrackError as TTE;
use error::TimeTrackErrorKind as TTEK;
use error::MapErrInto;

use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagstore::storeid::StoreIdIterator;
use libimagerror::into::IntoError;

use iter::get::GetTimeTrackIter;
use tag::TimeTrackingTag as TTT;
use timetracking::TimeTracking;

pub struct WithOneOf<'a> {
    iter: GetTimeTrackIter<'a>,
    allowed_tags: &'a Vec<TTT>,
}

impl<'a> WithOneOf<'a> {

    pub fn new(iter: GetTimeTrackIter<'a>, allowed_tags: &'a Vec<TTT>) -> WithOneOf<'a> {
        WithOneOf {
            iter: iter,
            allowed_tags: allowed_tags
        }
    }
}

impl<'a> Iterator for WithOneOf<'a> {
    type Item = Result<FileLockEntry<'a>, TTE>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Ok(fle)) => {
                    match fle.get_timetrack_tag() {
                        Err(e) => return Some(Err(e)),
                        Ok(t) => if self.allowed_tags.contains(&t) {
                            return Some(Ok(fle))
                        } else {
                            // loop
                        },
                    }
                },
                Some(Err(e)) => return Some(Err(e)),
                None => return None,
            }
        }
    }
}

