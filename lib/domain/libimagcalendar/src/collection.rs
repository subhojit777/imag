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

use std::path::Path;

use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::Store;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagentrylink::internal::InternalLinker;

use store::calendars::CalendarStore;
use store::iter::CalendarIter;
use error::CalendarError as CE;
use error::Result;

/// A Collection is a set of calendars
///
/// A Collection represents a directory on the filesystem where ical files are located
pub trait Collection {

    fn calendars(&self) -> Result<CalendarIter<StoreIdIterator>>;

    fn add_retrieve_calendar_from_path<'a, P: AsRef<Path>>(&mut self, p: P, store: &'a Store)
        -> Result<FileLockEntry<'a>>;

}

impl Collection for Entry {

    fn calendars(&self) -> Result<CalendarIter<StoreIdIterator>> {
        let i = self.get_internal_links()?.map(|l| l.get_store_id().clone());
        Ok(CalendarIter::new(StoreIdIterator::new(Box::new(i))))
    }

    /// Add a calendar to the collection
    ///
    /// # Internals
    ///
    /// Uses `Store::retrieve()` so if the calendar was already added to the store, this function
    /// loads that entry and ensures that the calendar is linked to the collection.
    ///
    fn add_retrieve_calendar_from_path<'a, P: AsRef<Path>>(&mut self, p: P, store: &'a Store)
        -> Result<FileLockEntry<'a>>
    {
        store.retrieve_calendar(p)
            .and_then(|mut fle| {
                self.add_internal_link(&mut fle)
                    .map_err(CE::from)
                    .map(|_| fle)
            })
    }
}

