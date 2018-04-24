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

use std::fmt::{Debug, Formatter, Error as FmtError};
use std::result::Result as RResult;

use filters::filter::Filter;

use libimagstore::storeid::StoreIdIterator;
use libimagstore::storeid::StoreId;

use is_in_diary::IsInDiary;
use error::DiaryErrorKind as DEK;
use error::DiaryError as DE;
use error::ResultExt;
use error::Result;

/// A iterator for iterating over diary entries
pub struct DiaryEntryIterator {
    name: String,
    iter: StoreIdIterator,

    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
}

impl Debug for DiaryEntryIterator {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "DiaryEntryIterator<name = {}, year = {:?}, month = {:?}, day = {:?}>",
               self.name, self.year, self.month, self.day)
    }

}

impl DiaryEntryIterator {

    pub fn new(diaryname: String, iter: StoreIdIterator) -> DiaryEntryIterator {
        DiaryEntryIterator {
            name: diaryname,
            iter: iter,

            year: None,
            month: None,
            day: None,
        }
    }

    // Filter by year, get all diary entries for this year
    pub fn year(mut self, year: i32) -> DiaryEntryIterator {
        self.year = Some(year);
        self
    }

    // Filter by month, get all diary entries for this month (every year)
    pub fn month(mut self, month: u32) -> DiaryEntryIterator {
        self.month = Some(month);
        self
    }

    // Filter by day, get all diary entries for this day (every year, every year)
    pub fn day(mut self, day: u32) -> DiaryEntryIterator {
        self.day = Some(day);
        self
    }

}

impl Filter<StoreId> for DiaryEntryIterator {
    fn filter(&self, id: &StoreId) -> bool {
        if id.is_in_diary(&self.name) {
            match (self.year, self.month, self.day) {
                (None    , None    , None)    => true,
                (Some(y) , None    , None)    => id.is_in_collection(&[&self.name, &y.to_string()]),
                (Some(y) , Some(m) , None)    => id.is_in_collection(&[&self.name, &y.to_string(), &m.to_string()]),
                (Some(y) , Some(m) , Some(d)) => id.is_in_collection(&[&self.name, &y.to_string(), &m.to_string(), &d.to_string()]),
                (None    , Some(_) , Some(_)) => false /* invalid case */,
                (None    , None    , Some(_)) => false /* invalid case */,
                (None    , Some(_) , None)    => false /* invalid case */,
                (Some(_) , None    , Some(_)) => false /* invalid case */,
            }
        } else {
            false
        }
    }
}

impl Iterator for DiaryEntryIterator {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None    => return None,
                Some(s) => {
                    debug!("Next element: {:?}", s);
                    if Filter::filter(self, &s) {
                        return Some(s)
                    } else {
                        continue
                    }
                },
            }
        }
    }
}


/// Get diary names.
///
/// # Warning
///
/// Does _not_ run a `unique` on the iterator!
pub struct DiaryNameIterator(StoreIdIterator);

impl DiaryNameIterator {
    pub fn new(s: StoreIdIterator) -> DiaryNameIterator {
        DiaryNameIterator(s)
    }
}

impl Iterator for DiaryNameIterator {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            if next.is_in_collection(&["diary"]) {
                return Some(next
                    .to_str()
                    .chain_err(|| DEK::DiaryNameFindingError)
                    .and_then(|s| {
                        s.split("diary/")
                            .nth(1)
                            .and_then(|n| n.split("/").nth(0).map(String::from))
                            .ok_or(DE::from_kind(DEK::DiaryNameFindingError))
                    }))
            }
        }

        None
    }

}

