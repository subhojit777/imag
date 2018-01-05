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

use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::IntoStoreId;

use group::CardGroup;
use iter::CardGroupIds;
use iter::SessionIds;
use error::Result;
use pathes::mk_group_path;

pub trait CardStore<'a> {
    fn new_group(&'a self, name: &String)         -> Result<FileLockEntry<'a>>;
    fn get_group_by_name(&'a self, name: &String) -> Result<Option<FileLockEntry<'a>>>;
    fn all_groups(&self)                          -> Result<CardGroupIds>;
    fn all_sessions(&self)                        -> Result<SessionIds>;
}

impl<'a> CardStore<'a> for Store {

    fn new_group(&'a self, name: &String) -> Result<FileLockEntry<'a>> {
        let id = ::module_path::ModuleEntryPath::new(mk_group_path(name)).into_storeid()?;
        self.create(id).map_err(From::from)
    }

    fn get_group_by_name(&'a self, name: &String) -> Result<Option<FileLockEntry<'a>>> {
        let id = ::module_path::ModuleEntryPath::new(mk_group_path(name)).into_storeid()?;
        self.get(id).map_err(From::from)
    }

    fn all_groups(&self) -> Result<CardGroupIds> {
        self.entries().map(|it| CardGroupIds::new(it.without_store())).map_err(From::from)
    }

    fn all_sessions(&self) -> Result<SessionIds> {
        self.entries().map(|it| SessionIds::new(it.without_store())).map_err(From::from)
    }

}
