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

use std::path::PathBuf;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::IntoStoreId;

use error::WikiError as WE;
use error::Result;

pub struct Wiki<'a, 'b>(&'a Store, &'b str);

impl<'a, 'b> Wiki<'a, 'b> {

    pub(crate) fn new(store: &'a Store, name: &'b str) -> Wiki<'a, 'b> {
        Wiki(store, name)
    }

    pub fn create_entry<EN: AsRef<str>>(&self, entry_name: EN) -> Result<FileLockEntry<'a>> {
        let path  = PathBuf::from(format!("{}/{}", self.1, entry_name.as_ref()));
        let sid   = ::module_path::ModuleEntryPath::new(path).into_storeid()?;
        self.0.create(sid).map_err(WE::from)
    }

    pub fn retrieve_entry<EN: AsRef<str>>(&self, entry_name: EN) -> Result<FileLockEntry<'a>> {
        let path  = PathBuf::from(format!("{}/{}", self.1, entry_name.as_ref()));
        let sid   = ::module_path::ModuleEntryPath::new(path).into_storeid()?;
        self.0.retrieve(sid).map_err(WE::from)
    }

}


