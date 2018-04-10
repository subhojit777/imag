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

use filters::filter::Filter;

use libimagstore::store::Store;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::IntoStoreId;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIteratorWithStore;

use error::WikiError as WE;
use error::Result;

pub struct Wiki<'a, 'b>(&'a Store, &'b str);

impl<'a, 'b> Wiki<'a, 'b> {

    pub(crate) fn new(store: &'a Store, name: &'b str) -> Wiki<'a, 'b> {
        Wiki(store, name)
    }

    pub fn get_entry<EN: AsRef<str>>(&self, entry_name: EN) -> Result<Option<FileLockEntry<'a>>> {
        let path  = PathBuf::from(format!("{}/{}", self.1, entry_name.as_ref()));
        let sid   = ::module_path::ModuleEntryPath::new(path).into_storeid()?;
        self.0.get(sid).map_err(WE::from)
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

    pub fn all_ids(&self) -> Result<WikiIdIterator> {
        let filter = IdIsInWikiFilter(self.1);
        Ok(WikiIdIterator(self.0.entries()?, filter))
    }

}

pub struct WikiIdIterator<'a>(StoreIdIteratorWithStore<'a>, IdIsInWikiFilter<'a>);

impl<'a> Iterator for WikiIdIterator<'a> {
    type Item = StoreId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            if self.1.filter(&next) {
                return Some(next)
            }
        }

        None
    }
}

pub struct IdIsInWikiFilter<'a>(&'a str);

impl<'a> IdIsInWikiFilter<'a> {
    pub fn new(wiki_name: &'a str) -> Self {
        IdIsInWikiFilter(wiki_name)
    }
}

impl<'a> Filter<StoreId> for IdIsInWikiFilter<'a> {
    fn filter(&self, id: &StoreId) -> bool {
        id.is_in_collection(&["wiki", &self.0])
    }
}

impl<'a> Filter<Entry> for IdIsInWikiFilter<'a> {
    fn filter(&self, e: &Entry) -> bool {
        e.get_location().is_in_collection(&["wiki", &self.0])
    }
}


