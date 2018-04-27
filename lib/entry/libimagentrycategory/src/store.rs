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

use toml_query::insert::TomlValueInsertExt;
use toml_query::read::TomlValueReadTypeExt;
use toml::Value;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagentryutil::isa::Is;

use error::CategoryErrorKind as CEK;
use error::CategoryError as CE;
use error::ResultExt;
use error::Result;
use iter::CategoryNameIter;
use category::IsCategory;

pub const CATEGORY_REGISTER_NAME_FIELD_PATH : &'static str = "category.register.name";

/// Extension on the Store to make it a register for categories
///
/// The register writes files to the
pub trait CategoryStore {

    fn category_exists(&self, name: &str) -> Result<bool>;

    fn create_category<'a>(&'a self, name: &str) -> Result<FileLockEntry<'a>>;

    fn delete_category(&self, name: &str) -> Result<()>;

    fn all_category_names(&self) -> Result<CategoryNameIter>;

    fn get_category_by_name(&self, name: &str) -> Result<Option<FileLockEntry>>;

}

impl CategoryStore for Store {

    /// Check whether a category exists
    fn category_exists(&self, name: &str) -> Result<bool> {
        trace!("Category exists? '{}'", name);
        let sid = mk_category_storeid(self.path().clone(), name)?;
        represents_category(self, sid, name)
    }

    /// Create a category
    ///
    /// Fails if the category already exists (returns false then)
    fn create_category<'a>(&'a self, name: &str) -> Result<FileLockEntry<'a>> {
        trace!("Creating category: '{}'", name);
        let sid         = mk_category_storeid(self.path().clone(), name)?;
        let mut entry   = self.create(sid)?;

        entry.set_isflag::<IsCategory>()?;

        let _   = entry
            .get_header_mut()
            .insert(CATEGORY_REGISTER_NAME_FIELD_PATH, Value::String(String::from(name)))?;

        trace!("Creating category worked: '{}'", name);
        Ok(entry)
    }

    /// Delete a category
    fn delete_category(&self, name: &str) -> Result<()> {
        trace!("Deleting category: '{}'", name);
        let sid = mk_category_storeid(self.path().clone(), name)?;
        self.delete(sid).map_err(CE::from)
    }

    /// Get all category names
    fn all_category_names(&self) -> Result<CategoryNameIter> {
        trace!("Getting all category names");
        Ok(CategoryNameIter::new(self, self.entries()?.without_store()))
    }

    /// Get a category by its name
    ///
    /// Returns the FileLockEntry which represents the category, so one can link to it and use it
    /// like a normal file in the store (which is exactly what it is).
    fn get_category_by_name(&self, name: &str) -> Result<Option<FileLockEntry>> {
        trace!("Getting category by name: '{}'", name);
        let sid = mk_category_storeid(self.path().clone(), name)?;

        self.get(sid)
            .chain_err(|| CEK::StoreWriteError)
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use std::path::PathBuf;

    use super::*;

    use libimagstore::store::Store;

    pub fn get_store() -> Store {
        use libimagstore::store::InMemoryFileAbstraction;
        let backend = Box::new(InMemoryFileAbstraction::default());
        Store::new_with_backend(PathBuf::from("/"), &None, backend).unwrap()
    }

    #[test]
    fn test_non_existing_category_exists() {
        let exists = get_store().category_exists("nonexistent");

        assert!(exists.is_ok(), format!("Expected Ok(_), got: {:?}", exists));
        let exists = exists.unwrap();

        assert!(!exists);
    }

    #[test]
    fn test_creating_category() {
        let category_name = "examplecategory";
        let store         = get_store();
        let res           = store.create_category(category_name);

        assert!(res.is_ok(), format!("Expected Ok(_), got: {:?}", res));
    }

    #[test]
    fn test_creating_category_creates_store_entry() {
        let category_name = "examplecategory";
        let store         = get_store();

        {
            let res = store.create_category(category_name);
            assert!(res.is_ok(), format!("Expected Ok(_), got: {:?}", res));
        }

        let category = store.get(PathBuf::from(format!("category/{}", category_name)));

        assert!(category.is_ok(), format!("Expected Ok(_), got: {:?}", category));
        let category = category.unwrap();

        assert!(category.is_some());
    }

    #[test]
    fn test_creating_category_creates_store_entry_with_header_field_set() {
        let _ = env_logger::try_init();
        let category_name = "examplecategory";
        let store         = get_store();

        {
            let res = store.create_category(category_name);
            assert!(res.is_ok(), format!("Expected Ok(_), got: {:?}", res));
        }

        let id = PathBuf::from(format!("category/{}", category_name));
        println!("Trying: {:?}", id);
        let category = store.get(id);

        assert!(category.is_ok(), format!("Expected Ok(_), got: {:?}", category));
        let category = category.unwrap();

        assert!(category.is_some());
        let category = category.unwrap();

        let header_field = category.get_header().read_string(CATEGORY_REGISTER_NAME_FIELD_PATH);
        assert!(header_field.is_ok(), format!("Expected Ok(_), got: {:?}", header_field));
        let header_field = header_field.unwrap();

        match header_field {
            Some(ref s) => assert_eq!(category_name, s),
            None        => assert!(false, "Header field not present"),
        }
    }
}

#[inline]
fn mk_category_storeid(base: PathBuf, s: &str) -> Result<StoreId> {
    use libimagstore::storeid::IntoStoreId;
    ::module_path::ModuleEntryPath::new(s)
        .into_storeid()
        .map(|id| id.with_base(base))
        .chain_err(|| CEK::StoreIdHandlingError)
}

#[inline]
fn represents_category(store: &Store, sid: StoreId, name: &str) -> Result<bool> {
    sid.exists()
        .chain_err(|| CEK::StoreIdHandlingError)
        .and_then(|bl| {
            if bl {
                store.get(sid)
                    .chain_err(|| CEK::StoreReadError)
                    .and_then(|fle| {
                        if let Some(fle) = fle {
                            fle.get_header()
                                .read_string(&String::from(CATEGORY_REGISTER_NAME_FIELD_PATH))
                                .chain_err(|| CEK::HeaderReadError)?
                                .ok_or(CE::from_kind(CEK::TypeError))
                                .map(|s| s == name)
                        } else {
                            Ok(false)
                        }
                    })
            } else {
                Ok(bl) // false
            }
        })
}

