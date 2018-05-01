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

use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;

use toml_query::read::TomlValueReadTypeExt;

use error::Result;
use error::CategoryError as CE;
use error::CategoryErrorKind as CEK;
use store::CATEGORY_REGISTER_NAME_FIELD_PATH;
use entry::EntryCategory;
use error::ResultExt;

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

    pub(crate) fn new(store: &'a Store, sidit: StoreIdIterator) -> CategoryNameIter<'a> {
        CategoryNameIter(store, sidit)
    }

}

impl<'a> Iterator for CategoryNameIter<'a> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Optimize me with lazy_static
        let query = CATEGORY_REGISTER_NAME_FIELD_PATH;

        while let Some(sid) = self.1.next() {
            match sid {
                Err(e) => return Some(Err(e).map_err(CE::from)),
                Ok(sid) => {
                    if sid.is_in_collection(&["category"]) {
                        let func = |store: &Store| { // hack for returning Some(Result<_, _>)
                            store
                                .get(sid)?
                                .ok_or_else(|| CE::from_kind(CEK::StoreReadError))?
                                .get_header()
                                .read_string(query)
                                .chain_err(|| CEK::HeaderReadError)?
                                .ok_or_else(|| CE::from_kind(CEK::StoreReadError))
                        };

                    return Some(func(&self.0))
                    }
                },
            } // else continue
        }

        None
    }
}

pub struct CategoryEntryIterator<'a>(&'a Store, StoreIdIterator, String);

impl<'a> CategoryEntryIterator<'a> {
    pub(crate) fn new(store: &'a Store, sit: StoreIdIterator, name: String) -> Self {
        CategoryEntryIterator(store, sit, name)
    }
}

impl<'a> Iterator for CategoryEntryIterator<'a> {
    type Item = Result<FileLockEntry<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.1.next() {
            match next {
                Err(e) => return Some(Err(e).map_err(CE::from)),
                Ok(next) => {
                    let getter = |next| -> Result<(String, FileLockEntry<'a>)> {
                        let entry = self.0
                            .get(next)?
                            .ok_or_else(|| CE::from_kind(CEK::StoreReadError))?;
                        Ok((entry.get_category()?, entry))
                    };

                    match getter(next) {
                        Err(e)     => return Some(Err(e)),
                        Ok((c, e)) => {
                            if c == self.2 {
                                return Some(Ok(e))
                            // } else {
                            // continue
                            }
                        }
                    }
                }
            }
        }

        None
    }
}
