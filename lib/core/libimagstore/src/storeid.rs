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

use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

use std::fmt::{Display, Debug, Formatter};
use std::fmt::Error as FmtError;
use std::result::Result as RResult;
use std::path::Components;

use error::StoreErrorKind as SEK;
use error::StoreError as SE;
use error::ResultExt;
use store::Result;
use store::Store;

use iter::create::StoreCreateIterator;
use iter::delete::StoreDeleteIterator;
use iter::get::StoreGetIterator;
use iter::retrieve::StoreRetrieveIterator;

/// The Index into the Store
#[derive(Debug, Clone, Hash, Eq, PartialOrd, Ord)]
pub struct StoreId {
    base: Option<PathBuf>,
    id:   PathBuf,
}

impl PartialEq for StoreId {
    fn eq(&self, other: &StoreId) -> bool {
        self.id == other.id
    }
}

impl StoreId {

    pub fn new(base: Option<PathBuf>, id: PathBuf) -> Result<StoreId> {
        StoreId::new_baseless(id).map(|mut sid| { sid.base = base; sid })
    }

    /// Try to create a StoreId object from a filesystem-absolute path.
    ///
    /// Automatically creates a StoreId object which has a `base` set to `store_part` if stripping
    /// the `store_part` from the `full_path` succeeded.
    ///
    /// Returns a `StoreErrorKind::StoreIdBuildFromFullPathError` if stripping failes.
    pub fn from_full_path<D>(store_part: &PathBuf, full_path: D) -> Result<StoreId>
        where D: Deref<Target = Path>
    {
        let p = full_path
            .strip_prefix(store_part)
            .chain_err(|| SEK::StoreIdBuildFromFullPathError)?;
        StoreId::new(Some(store_part.clone()), PathBuf::from(p))
    }

    pub fn new_baseless(id: PathBuf) -> Result<StoreId> {
        debug!("Trying to get a new baseless id from: {:?}", id);
        if id.is_absolute() {
            debug!("Error: Id is absolute!");
            Err(SE::from_kind(SEK::StoreIdLocalPartAbsoluteError(id)))
        } else {
            debug!("Building Storeid object baseless");
            Ok(StoreId {
                base: None,
                id
            })
        }
    }

    pub fn without_base(mut self) -> StoreId {
        self.base = None;
        self
    }

    pub fn with_base(mut self, base: PathBuf) -> Self {
        self.base = Some(base);
        self
    }

    /// Transform the StoreId object into a PathBuf, error if the base of the StoreId is not
    /// specified.
    pub fn into_pathbuf(mut self) -> Result<PathBuf> {
        let base = self.base.take();
        let mut base = base.ok_or_else(|| SEK::StoreIdHasNoBaseError(self.id.clone()))?;
        base.push(self.id);
        Ok(base)
    }

    pub fn exists(&self) -> Result<bool> {
        self.clone().into_pathbuf().map(|pb| pb.exists())
    }

    pub fn to_str(&self) -> Result<String> {
        self.base
            .as_ref()
            .cloned()
            .map(|mut base| { base.push(self.id.clone()); base })
            .unwrap_or_else(|| self.id.clone())
            .to_str()
            .map(String::from)
            .ok_or_else(|| SE::from_kind(SEK::StoreIdHandlingError))
    }

    /// Returns the components of the `id` part of the StoreId object.
    ///
    /// Can be used to check whether a StoreId points to an entry in a specific collection of
    /// StoreIds.
    pub fn components(&self) -> Components {
        self.id.components()
    }

    /// Get the _local_ part of a StoreId object, as in "the part from the store root to the entry".
    pub fn local(&self) -> &PathBuf {
        &self.id
    }

    /// Check whether a StoreId points to an entry in a specific collection.
    ///
    /// A "collection" here is simply a directory. So `foo/bar/baz` is an entry which is in
    /// collection ["foo", "bar", "baz"], but also in ["foo", "bar"] and ["foo"].
    ///
    /// # Warning
    ///
    /// The collection specification _has_ to start with the module name. Otherwise this function
    /// may return false negatives.
    ///
    pub fn is_in_collection<S: AsRef<str>, V: AsRef<[S]>>(&self, colls: &V) -> bool {
        use std::path::Component;

        self.id
            .components()
            .zip(colls.as_ref().iter())
            .map(|(component, pred_coll)| match component {
                Component::Normal(ref s) => s
                    .to_str()
                    .map(|ref s| s == &pred_coll.as_ref())
                    .unwrap_or(false),
                _ => false
            })
            .all(|x| x)
    }

    pub fn local_push<P: AsRef<Path>>(&mut self, path: P) {
        self.id.push(path)
    }

}

impl Display for StoreId {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        match self.id.to_str() {
            Some(s) => write!(fmt, "{}", s),
            None    => write!(fmt, "{}", self.id.to_string_lossy()),
        }
    }

}

/// This Trait allows you to convert various representations to a single one
/// suitable for usage in the Store
pub trait IntoStoreId {
    fn into_storeid(self) -> Result<StoreId>;
}

impl IntoStoreId for StoreId {
    fn into_storeid(self) -> Result<StoreId> {
        Ok(self)
    }
}

impl IntoStoreId for PathBuf {
    fn into_storeid(self) -> Result<StoreId> {
        StoreId::new_baseless(self)
    }
}

#[macro_export]
macro_rules! module_entry_path_mod {
    ($name:expr) => (
        #[deny(missing_docs,
                missing_copy_implementations,
                trivial_casts, trivial_numeric_casts,
                unsafe_code,
                unstable_features,
                unused_import_braces, unused_qualifications,
                unused_imports)]
        /// A helper module to create valid module entry paths
        pub mod module_path {
            use std::convert::AsRef;
            use std::path::Path;
            use std::path::PathBuf;

            use $crate::storeid::StoreId;
            use $crate::store::Result;

            /// A Struct giving you the ability to choose store entries assigned
            /// to it.
            ///
            /// It is created through a call to `new`.
            pub struct ModuleEntryPath(PathBuf);

            impl ModuleEntryPath {
                /// Path has to be a valid UTF-8 string or this will panic!
                pub fn new<P: AsRef<Path>>(pa: P) -> ModuleEntryPath {
                    let mut path = PathBuf::new();
                    path.push(format!("{}", $name));
                    path.push(pa.as_ref().clone());
                    let name = pa.as_ref().file_name().unwrap()
                        .to_str().unwrap();
                    path.set_file_name(name);
                    ModuleEntryPath(path)
                }
            }

            impl $crate::storeid::IntoStoreId for ModuleEntryPath {
                fn into_storeid(self) -> Result<$crate::storeid::StoreId> {
                    StoreId::new(None, self.0)
                }
            }
        }
    )
}

pub struct StoreIdIterator {
    iter: Box<Iterator<Item = Result<StoreId>>>,
}

impl Debug for StoreIdIterator {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "StoreIdIterator")
    }

}

impl StoreIdIterator {

    pub fn new(iter: Box<Iterator<Item = Result<StoreId>>>) -> StoreIdIterator {
        StoreIdIterator { iter }
    }

}

impl Iterator for StoreIdIterator {
    type Item = Result<StoreId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

}

pub struct StoreIdIteratorWithStore<'a>(StoreIdIterator, &'a Store);

impl<'a> Deref for StoreIdIteratorWithStore<'a> {
    type Target = StoreIdIterator;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Iterator for StoreIdIteratorWithStore<'a> {
    type Item = Result<StoreId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> StoreIdIteratorWithStore<'a> {

    pub fn new(iter: Box<Iterator<Item = Result<StoreId>>>, store: &'a Store) -> Self {
        StoreIdIteratorWithStore(StoreIdIterator::new(iter), store)
    }

    pub fn without_store(self) -> StoreIdIterator {
        self.0
    }

    /// Transform the iterator into a StoreCreateIterator
    ///
    /// This immitates the API from `libimagstore::iter`.
    pub fn into_create_iter(self) -> StoreCreateIterator<'a> {
        StoreCreateIterator::new(Box::new(self.0), self.1)
    }

    /// Transform the iterator into a StoreDeleteIterator
    ///
    ///
    /// This immitates the API from `libimagstore::iter`.
    pub fn into_delete_iter(self) -> StoreDeleteIterator<'a> {
        StoreDeleteIterator::new(Box::new(self.0), self.1)
    }

    /// Transform the iterator into a StoreGetIterator
    ///
    ///
    /// This immitates the API from `libimagstore::iter`.
    pub fn into_get_iter(self) -> StoreGetIterator<'a> {
        StoreGetIterator::new(Box::new(self.0), self.1)
    }

    /// Transform the iterator into a StoreRetrieveIterator
    ///
    ///
    /// This immitates the API from `libimagstore::iter`.
    pub fn into_retrieve_iter(self) -> StoreRetrieveIterator<'a> {
        StoreRetrieveIterator::new(Box::new(self.0), self.1)
    }

}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use storeid::StoreId;
    use storeid::IntoStoreId;
    use error::StoreErrorKind as SEK;

    module_entry_path_mod!("test");

    #[test]
    fn test_correct_path() {
        let p = module_path::ModuleEntryPath::new("test");

        assert_eq!(p.into_storeid().unwrap().to_str().unwrap(), "test/test");
    }

    #[test]
    fn test_baseless_path() {
        let id = StoreId::new_baseless(PathBuf::from("test"));
        assert!(id.is_ok());
        assert_eq!(id.unwrap(), StoreId {
            base: None,
            id: PathBuf::from("test")
        });
    }

    #[test]
    fn test_base_path() {
        let id = StoreId::from_full_path(&PathBuf::from("/tmp/"), PathBuf::from("/tmp/test"));
        assert!(id.is_ok());
        assert_eq!(id.unwrap(), StoreId {
            base: Some(PathBuf::from("/tmp/")),
            id: PathBuf::from("test")
        });
    }

    #[test]
    fn test_adding_base_to_baseless_path() {
        let id = StoreId::new_baseless(PathBuf::from("test"));

        assert!(id.is_ok());
        let id = id.unwrap();

        assert_eq!(id, StoreId { base: None, id: PathBuf::from("test") });

        let id = id.with_base(PathBuf::from("/tmp/"));
        assert_eq!(id, StoreId {
            base: Some(PathBuf::from("/tmp/")),
            id: PathBuf::from("test")
        });
    }

    #[test]
    fn test_removing_base_from_base_path() {
        let id = StoreId::from_full_path(&PathBuf::from("/tmp/"), PathBuf::from("/tmp/test"));

        assert!(id.is_ok());
        let id = id.unwrap();

        assert_eq!(id, StoreId {
            base: Some(PathBuf::from("/tmp/")),
            id: PathBuf::from("test")
        });

        let id = id.without_base();
        assert_eq!(id, StoreId {
            base: None,
            id: PathBuf::from("test")
        });
    }

    #[test]
    fn test_baseless_into_pathbuf_is_err() {
        let id = StoreId::new_baseless(PathBuf::from("test"));
        assert!(id.is_ok());
        assert!(id.unwrap().into_pathbuf().is_err());
    }

    #[test]
    fn test_baseless_into_pathbuf_is_storeidhasnobaseerror() {
        let id = StoreId::new_baseless(PathBuf::from("test"));
        assert!(id.is_ok());

        let pb = id.unwrap().into_pathbuf();
        assert!(pb.is_err());

        assert!(is_match!(pb.unwrap_err().kind(), &SEK::StoreIdHasNoBaseError(_)));
    }

    #[test]
    fn test_basefull_into_pathbuf_is_ok() {
        let id = StoreId::from_full_path(&PathBuf::from("/tmp/"), PathBuf::from("/tmp/test"));
        assert!(id.is_ok());
        assert!(id.unwrap().into_pathbuf().is_ok());
    }

    #[test]
    fn test_basefull_into_pathbuf_is_correct() {
        let id = StoreId::from_full_path(&PathBuf::from("/tmp/"), PathBuf::from("/tmp/test"));
        assert!(id.is_ok());

        let pb = id.unwrap().into_pathbuf();
        assert!(pb.is_ok());

        assert_eq!(pb.unwrap(), PathBuf::from("/tmp/test"));
    }

    #[test]
    fn storeid_in_collection() {
        let p = module_path::ModuleEntryPath::new("1/2/3/4/5/6/7/8/9/0").into_storeid().unwrap();

        assert!(p.is_in_collection(&["test", "1"]));
        assert!(p.is_in_collection(&["test", "1", "2"]));
        assert!(p.is_in_collection(&["test", "1", "2", "3"]));
        assert!(p.is_in_collection(&["test", "1", "2", "3", "4"]));
        assert!(p.is_in_collection(&["test", "1", "2", "3", "4", "5"]));
        assert!(p.is_in_collection(&["test", "1", "2", "3", "4", "5", "6"]));
        assert!(p.is_in_collection(&["test", "1", "2", "3", "4", "5", "6", "7"]));
        assert!(p.is_in_collection(&["test", "1", "2", "3", "4", "5", "6", "7", "8"]));
        assert!(p.is_in_collection(&["test", "1", "2", "3", "4", "5", "6", "7", "8", "9"]));
        assert!(p.is_in_collection(&["test", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"]));

        assert!(!p.is_in_collection(&["test", "0", "2", "3", "4", "5", "6", "7", "8", "9", "0"]));
        assert!(!p.is_in_collection(&["test", "1", "2", "3", "4", "5", "6", "8"]));
        assert!(!p.is_in_collection(&["test", "1", "2", "3", "leet", "5", "6", "7"]));
    }

}
