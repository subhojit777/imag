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

use libimagstore::storeid::StoreIdIterator;
use libimagstore::storeid::StoreIdIteratorWithStore;
use libimagstore::storeid::StoreId;

use util::IsHabitCheck;

pub struct HabitTemplateStoreIdIterator(StoreIdIterator);

impl Iterator for HabitTemplateStoreIdIterator {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(n) = self.0.next() {
            if n.is_habit_template() {
                return Some(n)
            }
        }
        None
    }
}

impl From<StoreIdIterator> for HabitTemplateStoreIdIterator {
    fn from(sii: StoreIdIterator) -> Self {
        HabitTemplateStoreIdIterator(sii)
    }
}

impl<'a> From<StoreIdIteratorWithStore<'a>> for HabitTemplateStoreIdIterator {
    fn from(sii: StoreIdIteratorWithStore<'a>) -> Self {
        HabitTemplateStoreIdIterator(sii.without_store())
    }
}

pub struct HabitInstanceStoreIdIterator(StoreIdIterator);

impl HabitInstanceStoreIdIterator {
    pub fn new(sid: StoreIdIterator) -> HabitInstanceStoreIdIterator {
        HabitInstanceStoreIdIterator(sid)
    }
}

impl Iterator for HabitInstanceStoreIdIterator {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(n) = self.0.next() {
            if n.is_habit_instance() {
                return Some(n)
            }
        }
        None
    }
}

impl From<StoreIdIterator> for HabitInstanceStoreIdIterator {
    fn from(sii: StoreIdIterator) -> Self {
        HabitInstanceStoreIdIterator(sii)
    }
}

impl<'a> From<StoreIdIteratorWithStore<'a>> for HabitInstanceStoreIdIterator {
    fn from(sii: StoreIdIteratorWithStore<'a>) -> Self {
        HabitInstanceStoreIdIterator(sii.without_store())
    }
}

