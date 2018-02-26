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

//! Store extensions
//!
//! The Store extension "CalendarStore" provides access to
//!
//! * Collections of Calendars
//! * Calendars
//!
//! But the store extension itself only provides getters for interfaces, because of the complexity
//! of this API.
//!

pub mod calendars;
pub mod collections;

use std::path::Path;

use self::calendars::CalendarStore;
use self::collections::CalendarCollectionStore;
use error::Result;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;


/// A CalendarDataStore provides getters for actual interfaces to calendar data
pub trait CalendarDataStore<'a> {

    /// Get an object which can be used to access collections of calendars
    fn calendar_collections(&self) -> CalendarCollectionStore<'a>;

    /// Get an object which can be used to access calendars
    fn calendars(&self) -> CalendarStore<'a>;

}

impl<'a> CalendarDataStore<'a> for Store {

    fn calendar_collections(&self) -> CalendarCollectionStore<'a> {
        unimplemented!()
    }

    fn calendars(&self) -> CalendarStore<'a> {
        unimplemented!()
    }

}

/// A abstract CRUD interface for retrieving calendar data from the Store
pub trait CalendarCRUD<'a> {

    fn get<H: AsRef<str>>(&self, hash: H)       -> Result<Option<FileLockEntry<'a>>>;
    fn create<P: AsRef<Path>>(&self, p: P)      -> Result<FileLockEntry<'a>>;
    fn retrieve<P: AsRef<Path>>(&self, p: P)    -> Result<FileLockEntry<'a>>;
    fn delete_by_hash(&self, hash: String)      -> Result<()>;

}

