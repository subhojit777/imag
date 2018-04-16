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

use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;

use error::WikiError as WE;
use error::Result;
use wiki::Wiki;

pub trait WikiStore {

    fn get_wiki<'a, 'b>(&'a self, name: &'b str) -> Result<Option<Wiki<'a, 'b>>>;

    fn create_wiki<'a, 'b>(&'a self, name: &'b str, mainpagename: Option<&str>)
        -> Result<Wiki<'a, 'b>>;

    fn retrieve_wiki<'a, 'b>(&'a self, name: &'b str, mainpagename: Option<&str>)
        -> Result<Wiki<'a, 'b>>;

    fn delete_wiki<N: AsRef<str>>(&self, name: N) -> Result<()>;

}

impl WikiStore for Store {

    /// get a wiki by its name
    fn get_wiki<'a, 'b>(&'a self, name: &'b str) -> Result<Option<Wiki<'a, 'b>>> {
        if wiki_path(name.as_ref())?.with_base(self.path().clone()).exists()? {
            debug!("Building Wiki object");
            Ok(Some(Wiki::new(self, name)))
        } else {
            debug!("Cannot build wiki object: Wiki does not exist");
            Ok(None)
        }
    }

    /// Create a wiki.
    ///
    /// # Returns
    ///
    /// Returns the Wiki object.
    ///
    /// Ob success, an empty Wiki entry with the name `mainpagename` (or "main" if none is passed)
    /// is created inside the wiki.
    ///
    fn create_wiki<'a, 'b>(&'a self, name: &'b str, mainpagename: Option<&str>)
        -> Result<Wiki<'a, 'b>>
    {
        debug!("Trying to get wiki '{}'", name);
        debug!("Trying to create wiki '{}' with mainpage: '{:?}'", name, mainpagename);

        let wiki = Wiki::new(self, name);
        let _    = wiki.create_index_page()?;

        wiki.create_entry(mainpagename.unwrap_or("main"))
            .map(|_| wiki)
    }

    fn retrieve_wiki<'a, 'b>(&'a self, name: &'b str, mainpagename: Option<&str>)
        -> Result<Wiki<'a, 'b>>
    {
        match self.get_wiki(name)? {
            None       => self.create_wiki(name, mainpagename),
            Some(wiki) => {
                let _ = wiki.retrieve_entry(mainpagename.unwrap_or("main"))?; // to make sure the page exists
                Ok(wiki)
            }
        }
    }

    /// Delete a wiki and all entries inside
    fn delete_wiki<N: AsRef<str>>(&self, name: N) -> Result<()> {
        unimplemented!()
    }

}

fn wiki_path(name: &str) -> Result<StoreId> {
    ::module_path::ModuleEntryPath::new(name).into_storeid().map_err(WE::from)
}

