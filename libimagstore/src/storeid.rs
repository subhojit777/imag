use std::path::PathBuf;

use std::fmt::{Display, Debug, Formatter};
use std::fmt::Error as FmtError;
use std::result::Result as RResult;
use std::path::Components;

use libimagerror::into::IntoError;

use error::StoreErrorKind as SEK;
use store::Result;
use store::Store;

/// The Index into the Store
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct StoreId {
    base: Option<PathBuf>,
    id:   PathBuf,
}

impl StoreId {

    pub fn new(base: Option<PathBuf>, id: PathBuf) -> Result<StoreId> {
        StoreId::new_baseless(id).map(|mut sid| { sid.base = base; sid })
    }

    pub fn new_baseless(id: PathBuf) -> Result<StoreId> {
        if id.is_absolute() {
            Err(SEK::StoreIdLocalPartAbsoluteError.into_error())
        } else {
            Ok(StoreId {
                base: None,
                id: id
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

    pub fn storified(self, store: &Store) -> StoreId {
        StoreId {
            base: Some(store.path().clone()),
            id: self.id
        }
    }

    pub fn exists(&self) -> bool {
        let pb : PathBuf = self.clone().into();
        pb.exists()
    }

    pub fn is_file(&self) -> bool {
        true
    }

    pub fn is_dir(&self) -> bool {
        false
    }

    pub fn to_str(&self) -> Result<String> {
        if self.base.is_some() {
            let mut base = self.base.as_ref().cloned().unwrap();
            base.push(self.id.clone());
            base
        } else {
            self.id.clone()
        }
        .to_str()
        .map(String::from)
        .ok_or(SEK::StoreIdHandlingError.into_error())
    }

    /// Returns the components of the `id` part of the StoreId object.
    ///
    /// Can be used to check whether a StoreId points to an entry in a specific collection of
    /// StoreIds.
    pub fn components(&self) -> Components {
        self.id.components()
    }

    /// Check whether a StoreId points to an entry in a specific collection.
    ///
    /// A "collection" here is simply a directory. So `foo/bar/baz` is an entry which is in
    /// collection ["foo", "bar"].
    ///
    /// # Warning
    ///
    /// The collection specification _has_ to start with the module name. Otherwise this function
    /// may return false negatives.
    ///
    pub fn is_in_collection(&self, colls: &[&str]) -> bool {
        use std::path::Component;

        self.id
            .components()
            .zip(colls)
            .map(|(component, pred_coll)| match component {
                Component::Normal(ref s) => s.to_str().map(|ref s| s == pred_coll).unwrap_or(false),
                _ => false
            })
            .all(|x| x) && colls.last().map(|last| !self.id.ends_with(last)).unwrap_or(false)
    }

}

impl Into<PathBuf> for StoreId {

    fn into(self) -> PathBuf {
        let mut base = self.base.unwrap_or(PathBuf::from("/"));
        base.push(self.id);
        base
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
    iter: Box<Iterator<Item = StoreId>>,
}

impl Debug for StoreIdIterator {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "StoreIdIterator")
    }

}

impl StoreIdIterator {

    pub fn new(iter: Box<Iterator<Item = StoreId>>) -> StoreIdIterator {
        StoreIdIterator {
            iter: iter,
        }
    }

}

impl Iterator for StoreIdIterator {
    type Item = StoreId;

    fn next(&mut self) -> Option<StoreId> {
        self.iter.next()
    }

}

#[cfg(test)]
mod test {

    use storeid::IntoStoreId;

    module_entry_path_mod!("test");

    #[test]
    fn correct_path() {
        let p = module_path::ModuleEntryPath::new("test");

        assert_eq!(p.into_storeid().unwrap().to_str().unwrap(), "test/test");
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

        // "0" is the filename, not a collection
        assert!(!p.is_in_collection(&["test", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"]));

        assert!(!p.is_in_collection(&["test", "0", "2", "3", "4", "5", "6", "7", "8", "9", "0"]));
        assert!(!p.is_in_collection(&["test", "1", "2", "3", "4", "5", "6", "8"]));
        assert!(!p.is_in_collection(&["test", "1", "2", "3", "leet", "5", "6", "7"]));
    }

}
