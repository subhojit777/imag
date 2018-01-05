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

/// Iterator which gets all "group" files for all groups
pub struct CardGroupIds(StoreIdIterator);

impl CardGroupIds {
    pub fn new(inner: StoreIdIterator) -> Self {
        CardGroupIds(inner)
    }
}

impl Iterator for CardGroupIds {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            // is in collection "flashcard/groups" and ends with "group"
            if next.is_in_collection(&["flashcard", "groups"]) && next.local().ends_with("group") {
                return Some(next);
            }
        }

        None
    }
}

/// Iterator which gets all cards for a group
pub struct CardsInGroup {
    inner: StoreIdIterator,
    groupname: String,
}

impl CardsInGroup {
    pub(crate) fn new(inner: StoreIdIterator, groupname: String) -> Self {
        CardsInGroup { inner, groupname }
    }
}

impl Iterator for CardsInGroup {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.inner.next() {
            if next.is_in_collection(&["flashcard", "groups", &self.groupname, "cards"]) {
                return Some(next);
            }
        }

        None
    }
}

/// Iterator which gets all sessions
pub struct SessionIds(StoreIdIterator);

impl SessionIds {
    pub fn new(inner: StoreIdIterator) -> Self {
        SessionIds(inner)
    }
}

impl Iterator for SessionIds {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            if next.is_in_collection(&["flashcard", "sessions"]) {
                return Some(next);
            }
        }

        None
    }
}

/// Iterator which gets all sessions for a group
pub struct SessionsForGroup(StoreIdIterator, String);

impl SessionsForGroup {
    pub fn new (inner: StoreIdIterator, groupname: String) -> Self {
        SessionsForGroup(inner, groupname)
    }
}

impl Iterator for SessionsForGroup {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            if next.is_in_collection(&["flashcard", "sessions", &self.1]) {
                return Some(next);
            }
        }

        None
    }
}

