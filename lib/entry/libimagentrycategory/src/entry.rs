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

use toml_query::insert::TomlValueInsertExt;
use toml_query::read::TomlValueReadExt;
use toml_query::read::TomlValueReadTypeExt;
use toml::Value;

use libimagstore::store::Entry;
use libimagentrylink::internal::InternalLinker;

use error::CategoryErrorKind as CEK;
use error::CategoryError as CE;
use error::ResultExt;
use error::Result;
use store::CategoryStore;

pub trait EntryCategory {

    fn set_category(&mut self, s: &str) -> Result<()>;

    fn set_category_checked(&mut self, register: &CategoryStore, s: &str) -> Result<()>;

    fn get_category(&self) -> Result<String>;

    fn has_category(&self) -> Result<bool>;

}

impl EntryCategory for Entry {

    fn set_category(&mut self, s: &str) -> Result<()> {
        trace!("Setting category '{}' UNCHECKED", s);
        self.get_header_mut()
            .insert(&String::from("category.value"), Value::String(s.to_string()))
            .chain_err(|| CEK::HeaderWriteError)
            .map(|_| ())
    }

    /// Check whether a category exists before setting it.
    ///
    /// This function should be used by default over EntryCategory::set_category()!
    fn set_category_checked(&mut self, register: &CategoryStore, s: &str) -> Result<()> {
        trace!("Setting category '{}' checked", s);
        let mut category = register
            .get_category_by_name(s)?
            .ok_or_else(|| CE::from_kind(CEK::CategoryDoesNotExist))?;

        let _ = self.set_category(s)?;
        let _ = self.add_internal_link(&mut category)?;

        Ok(())
    }

    fn get_category(&self) -> Result<String> {
        trace!("Getting category from '{}'", self.get_location());
        self.get_header()
            .read_string("category.value")?
            .ok_or_else(|| CE::from_kind(CEK::CategoryNameMissing))
    }

    fn has_category(&self) -> Result<bool> {
        trace!("Has category? '{}'", self.get_location());
        self.get_header().read("category.value")
            .chain_err(|| CEK::HeaderReadError)
            .map(|x| x.is_some())
    }

}
