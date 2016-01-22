use std::path::PathBuf;

/// The Index into the Store
pub type StoreId = PathBuf;

/// This Trait allows you to convert various representations to a single one
/// suitable for usage in the Store
trait IntoStoreId {
    fn into_storeid(self) -> StoreId;
}

impl IntoStoreId for PathBuf {
    fn into_storeid(self) -> StoreId {
        self
    }
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
                    path.push(format!("/{}", $name));
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
                fn into_storeid(mut self) -> $crate::storeid::StoreId {
                    self.0
                }
            }
        }
    )
}

#[cfg(test)]
mod test {

    use storeid::IntoStoreId;

    module_entry_path_mod!("test", "0.2.0-alpha+leet1337");

    #[test]
    fn correct_path() {
        let p = module_path::ModuleEntryPath::new("test");

        assert_eq!(p.into_storeid().to_str().unwrap(),
        "/test/test~0.2.0-alpha+leet1337");
    }

}
