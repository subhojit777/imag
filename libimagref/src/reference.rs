//! The Ref object is a helper over the link functionality, so one is able to create references to
//! files outside of the imag store.

use std::path::PathBuf;
use std::ops::Deref;
use std::ops::DerefMut;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagstore::store::Store;
use libimagerror::into::IntoError;

use toml::Value;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

use error::RefErrorKind as REK;
use flags::RefFlags;
use result::Result;
use module_path::ModuleEntryPath;

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
    pub fn create(store: &'a Store, pb: PathBuf, flags: RefFlags) -> Result<Ref<'a>> {
        if !pb.exists() {
            return Err(REK::RefTargetDoesNotExist.into_error());
        }
        if flags.get_content_hashing() && pb.is_dir() {
            return Err(REK::RefTargetCannotBeHashed.into_error());
        }

        let (mut fle, content_hash, permissions, canonical_path) = { // scope to be able to fold
            try!(File::open(pb.clone())
                .map_err(Box::new)
                .map_err(|e| REK::RefTargetFileCannotBeOpened.into_error_with_cause(e))

                // If we were able to open this file,
                // we hash the contents of the file and return (file, hash)
                .and_then(|mut file| {
                    let opt_contenthash = if flags.get_content_hashing() {
                        Some(hash_file_contents(&mut file))
                    } else {
                        None
                    };

                    Ok((file, opt_contenthash))
                })

                // and then we get the permissions if we have to
                // and return (file, content hash, permissions)
                .and_then(|(file, opt_contenthash)| {
                    let opt_permissions = if flags.get_permission_tracking() {
                        Some(try!(file
                                  .metadata()
                                  .map(|md| md.permissions())
                                  .map_err(Box::new)
                                  .map_err(|e| REK::RefTargetCannotReadPermissions.into_error_with_cause(e))
                        ))
                    } else {
                        None
                    };

                    Ok((file, opt_contenthash, opt_permissions))
                })

                // and then we try to canonicalize the PathBuf, because we want to store a
                // canonicalized path
                // and return (file, content hash, permissions, canonicalized path)
                .and_then(|(file, opt_contenthash, opt_permissions)| {
                    pb.canonicalize()
                        .map(|can| (file, opt_contenthash, opt_permissions, can))
                        // if PathBuf::canonicalize() failed, build an error from the return value
                        .map_err(|e| REK::PathCanonicalizationError.into_error_with_cause(Box::new(e)))
                })

                // and then we hash the canonicalized path
                // and return (file, content hash, permissions, canonicalized path, path hash)
                .and_then(|(file, opt_contenthash, opt_permissions, can)| {
                    let path_hash = try!(Ref::hash_path(&can)
                        .map_err(Box::new)
                        .map_err(|e| REK::PathHashingError.into_error_with_cause(e))
                    );

                    Ok((file, opt_contenthash, opt_permissions, can, path_hash))
                })

                // and then we convert the PathBuf of the canonicalized path to a String to be able
                // to save it in the Ref FileLockEntry obj
                // and return
                // (file, content hash, permissions, canonicalized path as String, path hash)
                .and_then(|(file, opt_conhash, opt_perm, can, path_hash)| {
                    match can.to_str().map(String::from) {
                        // UTF convert error in PathBuf::to_str(),
                        None      => Err(REK::PathUTF8Error.into_error()),
                        Some(can) => Ok((file, opt_conhash, opt_perm, can, path_hash))
                    }
                })

                // and then we create the FileLockEntry in the Store
                // and return (filelockentry, content hash, permissions, canonicalized path)
                .and_then(|(file, opt_conhash, opt_perm, can, path_hash)| {
                    let fle = try!(store
                                   .create(ModuleEntryPath::new(path_hash))
                                   .map_err(Box::new)
                                   .map_err(|e| REK::StoreWriteError.into_error_with_cause(e))
                    );

                    Ok((fle, opt_conhash, opt_perm, can))
                })
            )
        };

        for tpl in [
                Some(("ref",             Value::Table(BTreeMap::new()))),
                Some(("ref.permissions", Value::Table(BTreeMap::new()))),

                Some(("ref.path", Value::String(canonical_path))),

                content_hash.map(|h| ("ref.content_hash", Value::String(h))),
                permissions.map(|p| ("ref.permissions.ro", Value::Boolean(p.readonly()))),
            ].into_iter()
        {
            match tpl {
                &Some((ref s, ref v)) => {
                    match fle.get_header_mut().insert(s, v.clone()) {
                        Ok(false) => {
                            let e = REK::HeaderFieldAlreadyExistsError.into_error();
                            let e = Box::new(e);
                            let e = REK::HeaderFieldWriteError.into_error_with_cause(e);
                            return Err(e);
                        },
                        Err(e) => {
                            let e = Box::new(e);
                            let e = REK::HeaderFieldWriteError.into_error_with_cause(e);
                            return Err(e);
                        },
                        _ => (),
                    }
                }
                &None => {
                    debug!("Not going to insert.");
                }
            }
        }

        Ok(Ref(fle))
    }

    /// Creates a Hash from a PathBuf by making the PathBuf absolute and then running a hash
    /// algorithm on it
    fn hash_path(pb: &PathBuf) -> Result<String> {
        use std::io::Read;
        use crypto::sha1::Sha1;
        use crypto::digest::Digest;

        match pb.to_str() {
            Some(s) => {
                let mut hasher = Sha1::new();
                hasher.input_str(s);
                Ok(hasher.result_str())
            },
            None => return Err(REK::PathUTF8Error.into_error()),
        }
    }

    /// check whether the pointer the Ref represents still points to a file which exists
    pub fn fs_link_exists(&self) -> Result<bool> {
        self.fs_file().map(|pathbuf| pathbuf.exists())
    }

    /// Alias for `r.fs_link_exists() && r.deref().is_file()`
    pub fn is_ref_to_file(&self) -> Result<bool> {
        self.fs_file().map(|pathbuf| pathbuf.is_file())
    }

    /// Alias for `r.fs_link_exists() && r.deref().is_dir()`
    pub fn is_ref_to_dir(&self) -> Result<bool> {
        self.fs_file().map(|pathbuf| pathbuf.is_dir())
    }

    /// Alias for `!Ref::fs_link_exists()`
    pub fn is_dangling(&self) -> Result<bool> {
        self.fs_link_exists().map(|b| !b)
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
    pub fn fs_file(&self) -> Result<PathBuf> {
        match self.0.get_header().read("ref.path") {
            Ok(Some(Value::String(ref s))) => Ok(PathBuf::from(s)),
            Ok(Some(_)) => Err(REK::HeaderTypeError.into_error()),
            Ok(None)    => Err(REK::HeaderFieldMissingError.into_error()),
            Err(e)      => Err(REK::StoreReadError.into_error_with_cause(Box::new(e))),
        }
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

fn hash_file_contents(f: &mut File) -> String {
    let mut hasher = Sha1::new();
    let mut s = String::new();
    f.read_to_string(&mut s);
    hasher.input_str(&s[..]);
    hasher.result_str()
}

