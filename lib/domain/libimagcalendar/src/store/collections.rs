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
use libimagentryref::generators::sha1::Sha1;
use libimagentryref::refstore::RefStore;
use libimagentryutil::isa::Is;

use error::Result;
use error::CalendarError as CE;
use collection::IsCalendarCollection;

make_unique_ref_path_generator! (
    pub CalendarCollectionPathHasher
    over Sha1
    => with error CE
    => with collection name "calendar/collection"
    => |path| {
        Sha1::hash_path(path).map_err(CE::from)
    }
);

pub trait CalendarCollectionStore<'a> {
    fn get_calendar_collection<H: AsRef<str>>(&'a self, hash: H)    -> Result<Option<FileLockEntry<'a>>>;
    fn create_calendar_collection<P: AsRef<Path>>(&'a self, p: P)   -> Result<FileLockEntry<'a>>;
    fn retrieve_calendar_collection<P: AsRef<Path>>(&'a self, p: P) -> Result<FileLockEntry<'a>>;
    fn delete_calendar_collection_by_hash(&'a self, hash: String)   -> Result<()>;
}

impl<'a> CalendarCollectionStore<'a> for Store {

    /// Get a calendar collection
    fn get_calendar_collection<H: AsRef<str>>(&'a self, hash: H) -> Result<Option<FileLockEntry<'a>>> {
        self.get_ref::<CalendarCollectionPathHasher, H>(hash).map_err(CE::from)
    }

    /// Create a calendar collection
    ///
    /// # TODO
    ///
    /// Verify that the path `p` is a directory
    fn create_calendar_collection<P: AsRef<Path>>(&'a self, p: P) -> Result<FileLockEntry<'a>> {
        let mut r = self.create_ref::<CalendarCollectionPathHasher, P>(p)?;
        r.set_isflag::<IsCalendarCollection>()?;
        Ok(r)
    }

    /// Get or create a calendar collection
    ///
    /// # TODO
    ///
    /// Verify that the path `p` is a directory
    fn retrieve_calendar_collection<P: AsRef<Path>>(&'a self, p: P) -> Result<FileLockEntry<'a>> {
        let mut r = self.retrieve_ref::<CalendarCollectionPathHasher, P>(p)?;
        r.set_isflag::<IsCalendarCollection>()?;
        Ok(r)
    }

    /// Delete a calendar collection
    fn delete_calendar_collection_by_hash(&'a self, hash: String) -> Result<()> {
        unimplemented!()
    }

}
