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

use libimagstore::storeid::StoreIdIterator;
use libimagstore::storeid::StoreId;
use libimagstore::store::Store;

pub struct CalendarCollectionIter<I>(I)
    where I: Iterator<Item = StoreId>;

impl<I> CalendarCollectionIter<I>
    where I: Iterator<Item = StoreId>
{
    pub fn new(i: I) -> Self {
        CalendarCollectionIter(i)
    }
}

impl<I> Iterator for CalendarCollectionIter<I>
    where I: Iterator<Item = StoreId>
{
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                None    => return None,
                Some(n) => if n.is_in_collection(&["calendar", "collection"]) {
                    return Some(n)
                } else {
                    // continue
                },
            }
        }
    }
}

pub struct CalendarIter<I>(I)
    where I: Iterator<Item = StoreId>;

impl<I> CalendarIter<I>
    where I: Iterator<Item = StoreId>
{
    pub fn new(i: I) -> Self {
        CalendarIter(i)
    }
}

impl<I> Iterator for CalendarIter<I>
    where I: Iterator<Item = StoreId>
{
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                None    => return None,
                Some(n) => {
                    let is_calendar_and_not_collection =
                        n.is_in_collection(&["calendar"]) &&
                        !n.is_in_collection(&["calendar", "collection"]);

                    if is_calendar_and_not_collection {
                        return Some(n)
                    } else {
                        // continue
                    }
                },
            }
        }
    }
}

