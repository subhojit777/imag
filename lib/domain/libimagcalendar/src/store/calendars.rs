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

use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagentryref::refstore::RefStore;
use libimagentryref::generators::sha1::Sha1;

use error::Result;
use error::CalendarError as CE;

make_unique_ref_path_generator! (
    pub CalendarHasher
    over Sha1
    => with error CE
    => with collection name "calendar"
    => |path| {
        Sha1::hash_n_bytes(path, 4096).map_err(CE::from)
    }
);

/// A interface to the store which offers CRUD functionality for calendars
pub trait CalendarStore<'a> {
    fn get_calendar<H: AsRef<str>>(&'a self, hash: H)    -> Result<Option<FileLockEntry<'a>>>;
    fn create_calendar<P: AsRef<Path>>(&'a self, p: P)   -> Result<FileLockEntry<'a>>;
    fn retrieve_calendar<P: AsRef<Path>>(&'a self, p: P) -> Result<FileLockEntry<'a>>;
    fn delete_calendar_by_hash(&'a self, hash: String)   -> Result<()>;
}

impl<'a> CalendarStore<'a> for Store {

    /// Get a calendar
    fn get_calendar<H: AsRef<str>>(&'a self, hash: H) -> Result<Option<FileLockEntry<'a>>> {
        self.get_ref::<CalendarHasher, H>(hash).map_err(CE::from)
    }

    /// Create a calendar
    ///
    /// # TODO
    ///
    /// Check whether the path `p` is a file, return error if not
    fn create_calendar<P: AsRef<Path>>(&'a self, p: P) -> Result<FileLockEntry<'a>> {
        self.create_ref::<CalendarHasher, P>(p).map_err(CE::from)
    }

    /// Get or create a calendar
    ///
    /// # TODO
    ///
    /// Check whether the path `p` is a file, return error if not
    fn retrieve_calendar<P: AsRef<Path>>(&'a self, p: P) -> Result<FileLockEntry<'a>> {
        self.retrieve_ref::<CalendarHasher, P>(p).map_err(CE::from)
    }

    /// Delete a calendar
    fn delete_calendar_by_hash(&'a self, hash: String) -> Result<()> {
        unimplemented!()
    }

}

