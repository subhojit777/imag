//! The Ref object is a helper over the link functionality, so one is able to create references to
//! files outside of the imag store.

use std::path::PathBuf;
use std::ops::Deref;
use std::ops::DerefMut;

use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagstore::store::Store;
use libimagerror::into::IntoError;

use toml::Value;

use error::RefErrorKind as REK;
use flags::RefFlags;
use result::Result;

pub struct Ref<'a>(FileLockEntry<'a>);

impl<'a> Ref<'a> {

    /// Try to get `si` as Ref object from the store
    pub fn get(store: &'a Store, si: StoreId) -> Result<Ref<'a>> {
        match store.get(si) {
            Err(e) => return Err(REK::StoreReadError.into_error_with_cause(Box::new(e))),
            Ok(None) => return Err(REK::RefNotInStore.into_error()),
            Ok(Some(fle)) => Ref::read_reference(&fle).map(|_| Ref(fle)),
        }
    }

    fn read_reference(fle: &FileLockEntry<'a>) -> Result<PathBuf> {
        match fle.get_header().read("ref.reference") {
            Ok(Some(Value::String(s))) => Ok(PathBuf::from(s)),
            Ok(Some(_)) => Err(REK::HeaderTypeError.into_error()),
            Ok(None)    => Err(REK::HeaderFieldMissingError.into_error()),
            Err(e)      => Err(REK::StoreReadError.into_error_with_cause(Box::new(e))),
        }
    }

    /// Create a Ref object which refers to `pb`
    pub fn create(store: &Store, pb: PathBuf, flags: RefFlags) -> Result<Ref<'a>> {
        unimplemented!()
    }

    /// Creates a Hash from a PathBuf by making the PathBuf absolute and then running a hash
    /// algorithm on it
    fn hash_path(pb: &PathBuf) -> String {
        unimplemented!()
    }

    /// check whether the pointer the Ref represents still points to a file which exists
    pub fn fs_link_exists(&self) -> bool {
        unimplemented!()
    }

    /// Alias for `!Ref::fs_link_exists()`
    pub fn is_dangling(&self) -> bool {
        !self.fs_link_exists()
    }

    /// check whether the pointer the Ref represents is valid
    /// This includes:
    ///     - Hashsum of the file is still the same as stored in the Ref
    ///     - file permissions are still valid
    pub fn fs_link_valid(&self) -> bool {
        unimplemented!()
    }

    /// Check whether the file permissions of the referenced file are equal to the stored
    /// permissions
    pub fn fs_link_valid_permissions(&self) -> bool {
        unimplemented!()
    }

    /// Check whether the Hashsum of the referenced file is equal to the stored hashsum
    pub fn fs_link_valid_hash(&self) -> bool {
        unimplemented!()
    }

    /// Update the Ref by re-checking the file from FS
    /// This errors if the file is not present or cannot be read()
    pub fn update_ref(&mut self) -> Result<()> {
        unimplemented!()
    }

    /// Get the path of the file which is reffered to by this Ref
    pub fn fs_file(&self) -> &PathBuf {
        unimplemented!()
    }

    /// Check whether there is a reference to the file at `pb`
    pub fn exists(store: &Store, pb: PathBuf) -> Result<bool> {
        unimplemented!()
    }

    /// Re-find a referenced file
    ///
    /// This function tries to re-find a ref by searching all directories in `search_roots` recursively
    /// for a file which matches the hash of the Ref `ref`.
    ///
    /// If `search_roots` is `None`, it starts at the filesystem root `/`.
    ///
    /// # Warning
    ///
    /// This option causes heavy I/O as it recursively searches the Filesystem.
    pub fn refind(&self, search_roots: Option<Vec<PathBuf>>) -> Option<PathBuf> {
        unimplemented!()
    }

}

impl<'a> Deref for Ref<'a> {
    type Target = FileLockEntry<'a>;

    fn deref(&self) -> &FileLockEntry<'a> {
        &self.0
    }

}

impl<'a> DerefMut for Ref<'a> {

    fn deref_mut(&mut self) -> &mut FileLockEntry<'a> {
        &mut self.0
    }

}
