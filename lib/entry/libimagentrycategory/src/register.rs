//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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
use toml_query::read::TomlValueReadExt;
use toml::Value;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIterator;
use libimagerror::into::IntoError;

use category::Category;
use error::CategoryErrorKind as CEK;
use error::MapErrInto;
use result::Result;

pub const CATEGORY_REGISTER_NAME_FIELD_PATH : &'static str = "category.register.name";

/// Extension on the Store to make it a register for categories
///
/// The register writes files to the
pub trait CategoryRegister {

    fn category_exists(&self, name: &str) -> Result<bool>;

    fn create_category(&self, name: &str) -> Result<bool>;

    fn delete_category(&self, name: &str) -> Result<()>;

    fn all_category_names(&self) -> Result<CategoryNameIter>;

    fn get_category_by_name(&self, name: &str) -> Result<Option<FileLockEntry>>;

}

impl CategoryRegister for Store {

    /// Check whether a category exists
    fn category_exists(&self, name: &str) -> Result<bool> {
        let sid = try!(mk_category_storeid(self.path().clone(), name));
        represents_category(self, sid, name)
    }

    /// Create a category
    ///
    /// Fails if the category already exists (returns false then)
    fn create_category(&self, name: &str) -> Result<bool> {
        use libimagstore::error::StoreErrorKind as SEK;

        let sid = try!(mk_category_storeid(self.path().clone(), name));


        match self.create(sid) {
            Ok(mut entry) => {
                let val = Value::String(String::from(name));
                entry.get_header_mut()
                    .insert(CATEGORY_REGISTER_NAME_FIELD_PATH, val)
                    .map(|opt| if opt.is_none() {
                        debug!("Setting category header worked")
                    } else {
                        warn!("Setting category header replaced existing value: {:?}", opt);
                    })
                    .map(|_| true)
                    .map_err_into(CEK::HeaderWriteError)
                    .map_err_into(CEK::StoreWriteError)
            }
            Err(store_error) => if is_match!(store_error.err_type(), SEK::EntryAlreadyExists) {
                Ok(false)
            } else {
                Err(store_error).map_err_into(CEK::StoreWriteError)
            }
        }
    }

    /// Delete a category
    fn delete_category(&self, name: &str) -> Result<()> {
        let sid = try!(mk_category_storeid(self.path().clone(), name));

        self.delete(sid).map_err_into(CEK::StoreWriteError)
    }

    /// Get all category names
    fn all_category_names(&self) -> Result<CategoryNameIter> {
        self.retrieve_for_module("category")
            .map_err_into(CEK::StoreReadError)
            .map(|iter| CategoryNameIter::new(self, iter))
    }

    /// Get a category by its name
    ///
    /// Returns the FileLockEntry which represents the category, so one can link to it and use it
    /// like a normal file in the store (which is exactly what it is).
    fn get_category_by_name(&self, name: &str) -> Result<Option<FileLockEntry>> {
        let sid = try!(mk_category_storeid(self.path().clone(), name));

        self.get(sid)
            .map_err_into(CEK::StoreWriteError)
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
        let backend = Box::new(InMemoryFileAbstraction::new());
        Store::new_with_backend(PathBuf::from("/"), None, backend).unwrap()
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
        let res = res.unwrap();
        assert!(res);
    }

    #[test]
    fn test_creating_category_creates_store_entry() {
        let category_name = "examplecategory";
        let store         = get_store();

        let res           = store.create_category(category_name);

        assert!(res.is_ok(), format!("Expected Ok(_), got: {:?}", res));
        let res = res.unwrap();
        assert!(res);

        let category = store.get(PathBuf::from(format!("category/{}", category_name)));

        assert!(category.is_ok(), format!("Expected Ok(_), got: {:?}", category));
        let category = category.unwrap();

        assert!(category.is_some());
    }

    #[test]
    fn test_creating_category_creates_store_entry_with_header_field_set() {
        let _ = env_logger::init();
        let category_name = "examplecategory";
        let store         = get_store();
        let res           = store.create_category(category_name);

        assert!(res.is_ok(), format!("Expected Ok(_), got: {:?}", res));
        let res = res.unwrap();
        assert!(res);

        let id = PathBuf::from(format!("category/{}", category_name));
        println!("Trying: {:?}", id);
        let category = store.get(id);

        assert!(category.is_ok(), format!("Expected Ok(_), got: {:?}", category));
        let category = category.unwrap();

        assert!(category.is_some());
        let category = category.unwrap();

        let header_field = category.get_header().read(CATEGORY_REGISTER_NAME_FIELD_PATH);
        assert!(header_field.is_ok(), format!("Expected Ok(_), got: {:?}", header_field));
        let header_field = header_field.unwrap();

        match header_field {
            Some(&Value::String(ref s)) => assert_eq!(category_name, s),
            Some(_) => assert!(false, "Header field has wrong type"),
            None    => assert!(false, "Header field not present"),
        }
    }
}

#[inline]
fn mk_category_storeid(base: PathBuf, s: &str) -> Result<StoreId> {
    use libimagstore::storeid::IntoStoreId;
    ::module_path::ModuleEntryPath::new(s)
        .into_storeid()
        .map(|id| id.with_base(base))
        .map_err_into(CEK::StoreIdHandlingError)
}

#[inline]
fn represents_category(store: &Store, sid: StoreId, name: &str) -> Result<bool> {
    sid.exists()
        .map_err_into(CEK::StoreIdHandlingError)
        .and_then(|bl| {
            if bl {
                store.get(sid)
                    .map_err_into(CEK::StoreReadError)
                    .and_then(|fle| {
                        if let Some(fle) = fle {
                            match fle.get_header()
                                .read(&String::from(CATEGORY_REGISTER_NAME_FIELD_PATH))
                                .map_err_into(CEK::HeaderReadError)
                            {
                                Ok(Some(&Value::String(ref s))) => Ok(s == name),
                                Ok(_)                     => Err(CEK::TypeError.into_error()),
                                Err(e)                    => Err(e).map_err_into(CEK::HeaderReadError),
                            }
                        } else {
                            Ok(false)
                        }
                    })
            } else {
                Ok(bl) // false
            }
        })
}

/// Iterator for Category names
///
/// Iterates over Result<Category>
///
/// # Return values
///
/// In each iteration, a Option<Result<Category>> is returned. Error kinds are as follows:
///
/// * CategoryErrorKind::StoreReadError if a name could not be fetched from the store
/// * CategoryErrorKind::HeaderReadError if the header of the fetched item couldn't be read
/// * CategoryErrorKind::TypeError if the name could not be fetched because it is not a String
///
pub struct CategoryNameIter<'a>(&'a Store, StoreIdIterator);

impl<'a> CategoryNameIter<'a> {

    fn new(store: &'a Store, sidit: StoreIdIterator) -> CategoryNameIter<'a> {
        CategoryNameIter(store, sidit)
    }

}

impl<'a> Iterator for CategoryNameIter<'a> {
    type Item = Result<Category>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Optimize me with lazy_static
        let query = String::from(CATEGORY_REGISTER_NAME_FIELD_PATH);

        self.1
            .next()
            .map(|sid| {
                self.0
                    .get(sid)
                    .map_err_into(CEK::StoreReadError)
                    .and_then(|fle| fle.ok_or(CEK::StoreReadError.into_error()))
                    .and_then(|fle| match fle.get_header().read(&query) {
                        Ok(Some(&Value::String(ref s))) => Ok(Category::from(s.clone())),
                        Ok(_)  => Err(CEK::TypeError.into_error()),
                        Err(e) => Err(e).map_err_into(CEK::HeaderReadError),
                    })
            })
    }
}

