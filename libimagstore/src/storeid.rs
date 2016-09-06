use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

use std::fmt::{Display, Debug, Formatter};
use std::fmt::Error as FmtError;
use std::result::Result as RResult;
use std::path::Components;

use libimagerror::into::IntoError;

use error::StoreErrorKind as SEK;
use error::MapErrInto;
use store::Result;

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

    /// Try to create a StoreId object from a filesystem-absolute path.
    ///
    /// Automatically creates a StoreId object which has a `base` set to `store_part` if stripping
    /// the `store_part` from the `full_path` succeeded.
    ///
    /// Returns a `StoreErrorKind::StoreIdBuildFromFullPathError` if stripping failes.
    pub fn from_full_path<D>(store_part: &PathBuf, full_path: D) -> Result<StoreId>
        where D: Deref<Target = Path>
    {
        let p = try!(
            full_path.strip_prefix(store_part).map_err_into(SEK::StoreIdBuildFromFullPathError)
        );
        StoreId::new(Some(store_part.clone()), PathBuf::from(p))
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

    /// Get the _local_ part of a StoreId object, as in "the part from the store root to the entry".
    pub fn local(&self) -> &PathBuf {
        &self.id
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

}
