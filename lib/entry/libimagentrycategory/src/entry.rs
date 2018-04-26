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

use error::CategoryErrorKind as CEK;
use error::CategoryError as CE;
use error::ResultExt;
use error::Result;
use register::CategoryRegister;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Category(String);

impl From<String> for Category {

    fn from(s: String) -> Category {
        Category(s)
    }

}

impl Into<String> for Category {
    fn into(self) -> String {
        self.0
    }
}

pub trait EntryCategory {

    fn set_category(&mut self, s: Category) -> Result<()>;

    fn set_category_checked(&mut self, register: &CategoryRegister, s: Category) -> Result<()>;

    fn get_category(&self) -> Result<Option<Category>>;

    fn has_category(&self) -> Result<bool>;

}

impl EntryCategory for Entry {

    fn set_category(&mut self, s: Category) -> Result<()> {
        self.get_header_mut()
            .insert(&String::from("category.value"), Value::String(s.into()))
            .chain_err(|| CEK::HeaderWriteError)
            .map(|_| ())
    }

    /// Check whether a category exists before setting it.
    ///
    /// This function should be used by default over EntryCategory::set_category()!
    fn set_category_checked(&mut self, register: &CategoryRegister, s: Category) -> Result<()> {
        let c_str        = s.clone().into();
        let mut category = register
            .get_category_by_name(&c_str)?
            .ok_or_else(|| CE::from_kind(CEK::CategoryDoesNotExist))?;

        let _ = self.set_category(s)?;
        let _ = self.add_internal_link(&mut category)?;

        Ok(())
    }

    fn get_category(&self) -> Result<Option<Category>> {
        self.get_header()
            .read_string("category.value")
            .chain_err(|| CEK::HeaderReadError)
            .and_then(|o| o.map(Category::from).ok_or(CE::from_kind(CEK::TypeError)))
            .map(Some)
    }

    fn has_category(&self) -> Result<bool> {
        self.get_header().read("category.value")
            .chain_err(|| CEK::HeaderReadError)
            .map(|x| x.is_some())
    }

}
