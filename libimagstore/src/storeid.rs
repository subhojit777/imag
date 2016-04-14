use std::path::PathBuf;
use std::path::Path;
use std::borrow::Borrow;

use glob::Paths;
use semver::Version;
use std::fmt::{Debug, Formatter};
use std::fmt::Error as FmtError;
use std::result::Result as RResult;

use error::{StoreError, StoreErrorKind};
use store::Result;
use store::Store;

/// The Index into the Store
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct StoreId(PathBuf);

impl Into<PathBuf> for StoreId {

    fn into(self) -> PathBuf {
        self.0
    }

}

impl From<PathBuf> for StoreId {

    fn from(pb: PathBuf) -> StoreId {
        StoreId(pb)
    }

}

impl From<String> for StoreId {

    fn from(string: String) -> StoreId {
        StoreId(string.into())
    }

}

impl AsRef<Path> for StoreId {

    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }

}

impl Borrow<Path> for StoreId {

    fn borrow(&self) -> &Path {
        self.0.borrow()
    }

}

/// This Trait allows you to convert various representations to a single one
/// suitable for usage in the Store
pub trait IntoStoreId {
    fn into_storeid(self) -> StoreId;
}

impl IntoStoreId for PathBuf {
    fn into_storeid(self) -> StoreId {
        self
    }
}

pub fn build_entry_path(store: &Store, path_elem: &str) -> Result<PathBuf> {
    debug!("Checking path element for version");
    if path_elem.split("~").last().map(|v| Version::parse(v).is_err()).unwrap_or(false) {
        debug!("Version cannot be parsed from {:?}", path_elem);
        debug!("Path does not contain version!");
        return Err(StoreError::new(StoreErrorKind::StorePathLacksVersion, None));
    }
    debug!("Version checking succeeded");

    debug!("Building path from {:?}", path_elem);
    let mut path = store.path().clone();

    if path_elem.chars().next() == Some('/') {
        path.push(&path_elem[1..path_elem.len()]);
    } else {
        path.push(path_elem);
    }

    Ok(path)
}

#[macro_export]
macro_rules! module_entry_path_mod {
    ($name:expr, $version:expr) => (
        #[deny(missing_docs,
                missing_copy_implementations,
                trivial_casts, trivial_numeric_casts,
                unsafe_code,
                unstable_features,
                unused_import_braces, unused_qualifications,
                unused_imports)]
        /// A helper module to create valid module entry paths
        pub mod module_path {
            use semver::Version;
            use std::convert::AsRef;
            use std::path::Path;
            use std::path::PathBuf;

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
                    let version = Version::parse($version).unwrap();
                    let name = pa.as_ref().file_name().unwrap()
                        .to_str().unwrap();
                    path.set_file_name(format!("{}~{}",
                                               name,
                                               version));
                    ModuleEntryPath(path)
                }
            }

            impl $crate::storeid::IntoStoreId for ModuleEntryPath {
                fn into_storeid(self) -> $crate::storeid::StoreId {
                    self.0
                }
            }
        }
    )
}

pub struct StoreIdIterator {
    paths: Paths,
}

impl Debug for StoreIdIterator {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "StoreIdIterator")
    }

}

impl StoreIdIterator {

    pub fn new(paths: Paths) -> StoreIdIterator {
        StoreIdIterator {
            paths: paths,
        }
    }

}

impl Iterator for StoreIdIterator {
    type Item = StoreId;

    fn next(&mut self) -> Option<StoreId> {
        self.paths.next().and_then(|o| o.ok())
    }

}

#[cfg(test)]
mod test {

    use storeid::IntoStoreId;

    module_entry_path_mod!("test", "0.2.0-alpha+leet1337");

    #[test]
    fn correct_path() {
        let p = module_path::ModuleEntryPath::new("test");

        assert_eq!(p.into_storeid().to_str().unwrap(), "test/test~0.2.0-alpha+leet1337");
    }

}
