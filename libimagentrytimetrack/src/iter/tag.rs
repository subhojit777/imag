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

use chrono::naive::NaiveDateTime as NDT;

use error::TimeTrackError;
use error::TimeTrackErrorKind as TTEK;
use tag::TimeTrackingTag as TTT;
use iter::storeid::TagStoreIdIter;

use libimagentrytag::tag::is_tag_str;
use libimagerror::into::IntoError;

pub struct TagIter(Box<Iterator<Item = String>>);

impl TagIter {
    pub fn new(i: Box<Iterator<Item = String>>) -> TagIter {
        TagIter(i)
    }

    pub fn create_storeids(self, datetime: NDT) -> TagStoreIdIter {
        TagStoreIdIter::new(self, datetime)
    }
}

impl Iterator for TagIter {
    type Item = Result<TTT, TimeTrackError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|t| if is_tag_str(&t).is_ok() {
                Ok(TTT::from(t))
            } else {
                Err(TTEK::TagFormat.into_error())
            })
    }
}

