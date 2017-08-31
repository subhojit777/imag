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
use std::collections::BTreeMap;
use std::fs::File;

use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagstore::store::Store;
use libimagerror::into::IntoError;

use toml::Value;

use error::RefErrorKind as REK;
use error::MapErrInto;
use flags::RefFlags;
use result::Result;
use hasher::*;
use module_path::ModuleEntryPath;
use util::*;

pub trait RefStore {

    /// Check whether there is a reference to the file at `pb`
    fn exists(&self, pb: PathBuf) -> Result<bool>;

    /// Try to get `si` as Ref object from the store
    fn get<'a>(&'a self, si: StoreId) -> Result<FileLockEntry<'a>>;

    /// Get a Ref object from the store by hash.
    ///
    /// Returns None if the hash cannot be found.
    fn get_by_hash<'a>(&'a self, hash: String) -> Result<Option<FileLockEntry<'a>>>;

    /// Delete a ref by hash
    ///
    /// If the returned Result contains an error, the ref might not be deleted.
    fn delete_by_hash(&self, hash: String) -> Result<()>;

    /// Create a Ref object which refers to `pb`
    fn create<'a>(&'a self, pb: PathBuf, flags: RefFlags) -> Result<FileLockEntry<'a>>;

    fn create_with_hasher<'a, H: Hasher>(&'a self, pb: PathBuf, flags: RefFlags, h: H)
        -> Result<FileLockEntry<'a>>;

}

impl RefStore for Store {

    /// Check whether there is a reference to the file at `pb`
    fn exists(&self, pb: PathBuf) -> Result<bool> {
        pb.canonicalize()
            .map_err_into(REK::PathCanonicalizationError)
            .and_then(|c| hash_path(&c))
            .map_err_into(REK::PathHashingError)
            .and_then(|hash| {
                self.retrieve_for_module("ref")
                    .map(|iter| (hash, iter))
                    .map_err_into(REK::StoreReadError)
            })
            .and_then(|(hash, possible_refs)| {
                // This is kind of a manual Iterator::filter() call what we do here, but with the
                // actual ::filter method we cannot return the error in a nice way, so we do it
                // manually here. If you can come up with a better version of this, feel free to
                // take this note as a todo.
                for r in possible_refs {
                    let contains_hash = try!(r.to_str()
                        .map_err_into(REK::TypeConversionError)
                        .map(|s| s.contains(&hash[..]))
                    );

                    if !contains_hash {
                        continue;
                    }

                    match self.get(r) {
                        Ok(Some(fle)) => {
                            if read_reference(&fle).map(|path| path == pb).unwrap_or(false) {
                                return Ok(true)
                            }
                        },

                        Ok(None) => return Err(REK::StoreReadError.into_error()),
                        Err(e)   => return Err(e).map_err_into(REK::StoreReadError)
                    }
                }

                Ok(false)
            })
    }

    /// Try to get `si` as Ref object from the store
    fn get<'a>(&'a self, si: StoreId) -> Result<FileLockEntry<'a>> {
        match self.get(si) {
            Err(e)        => return Err(e).map_err_into(REK::StoreReadError),
            Ok(None)      => return Err(REK::RefNotInStore.into_error()),
            Ok(Some(fle)) => Ok(fle),
        }
    }

    /// Get a Ref object from the store by hash.
    ///
    /// Returns None if the hash cannot be found.
    fn get_by_hash<'a>(&'a self, hash: String) -> Result<Option<FileLockEntry<'a>>> {
        ModuleEntryPath::new(hash)
            .into_storeid()
            .and_then(|id| self.get(id))
            .map_err_into(REK::StoreReadError)
    }

    /// Delete a ref by hash
    ///
    /// If the returned Result contains an error, the ref might not be deleted.
    fn delete_by_hash(&self, hash: String) -> Result<()> {
        ModuleEntryPath::new(hash)
            .into_storeid()
            .and_then(|id| self.delete(id))
            .map_err_into(REK::StoreWriteError)
    }

    /// Create a Ref object which refers to `pb`
    fn create<'a>(&'a self, pb: PathBuf, flags: RefFlags) -> Result<FileLockEntry<'a>> {
        self.create_with_hasher(pb, flags, DefaultHasher::new())
    }

    fn create_with_hasher<'a, H: Hasher>(&'a self, pb: PathBuf, flags: RefFlags, mut h: H)
        -> Result<FileLockEntry<'a>>
    {
        use toml_query::insert::TomlValueInsertExt;

        if !pb.exists() {
            return Err(REK::RefTargetDoesNotExist.into_error());
        }
        if flags.get_content_hashing() && pb.is_dir() {
            return Err(REK::RefTargetCannotBeHashed.into_error());
        }

        let (mut fle, content_hash, permissions, canonical_path) = { // scope to be able to fold
            try!(File::open(pb.clone())
                .map_err_into(REK::RefTargetFileCannotBeOpened)

                // If we were able to open this file,
                // we hash the contents of the file and return (file, hash)
                .and_then(|mut file| {
                    let opt_contenthash = if flags.get_content_hashing() {
                        Some(try!(h.create_hash(&pb, &mut file)))
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
                                  .map_err_into(REK::RefTargetCannotReadPermissions)
                        ))
                    } else {
                        None
                    };

                    Ok((opt_contenthash, opt_permissions))
                })

                // and then we try to canonicalize the PathBuf, because we want to store a
                // canonicalized path
                // and return (file, content hash, permissions, canonicalized path)
                .and_then(|(opt_contenthash, opt_permissions)| {
                    pb.canonicalize()
                        .map(|can| (opt_contenthash, opt_permissions, can))
                        // if PathBuf::canonicalize() failed, build an error from the return value
                        .map_err_into(REK::PathCanonicalizationError)
                })

                // and then we hash the canonicalized path
                // and return (file, content hash, permissions, canonicalized path, path hash)
                .and_then(|(opt_contenthash, opt_permissions, can)| {
                    let path_hash = try!(hash_path(&can).map_err_into(REK::PathHashingError));

                    Ok((opt_contenthash, opt_permissions, can, path_hash))
                })

                // and then we convert the PathBuf of the canonicalized path to a String to be able
                // to save it in the Ref FileLockEntry obj
                // and return
                // (file, content hash, permissions, canonicalized path as String, path hash)
                .and_then(|(opt_conhash, opt_perm, can, path_hash)| {
                    match can.to_str().map(String::from) {
                        // UTF convert error in PathBuf::to_str(),
                        None      => Err(REK::PathUTF8Error.into_error()),
                        Some(can) => Ok((opt_conhash, opt_perm, can, path_hash))
                    }
                })

                // and then we create the FileLockEntry in the Store
                // and return (filelockentry, content hash, permissions, canonicalized path)
                .and_then(|(opt_conhash, opt_perm, can, path_hash)| {
                    let fle = try!(self
                                   .create(ModuleEntryPath::new(path_hash))
                                   .map_err_into(REK::StoreWriteError)
                    );

                    Ok((fle, opt_conhash, opt_perm, can))
                })
            )
        };

        for tpl in [
                Some((String::from("ref"),              Value::Table(BTreeMap::new()))),
                Some((String::from("ref.permissions"),  Value::Table(BTreeMap::new()))),
                Some((String::from("ref.path"),         Value::String(canonical_path))),
                Some((String::from("ref.content_hash"), Value::Table(BTreeMap::new()))),

                content_hash.map(|hash| {
                    (format!("ref.content_hash.{}", h.hash_name()), Value::String(hash))
                }),
                permissions.map(|p| {
                    (String::from("ref.permissions.ro"), Value::Boolean(p.readonly()))
                }),
            ].into_iter()
        {
            match tpl {
                &Some((ref s, ref v)) => {
                    match fle.get_header_mut().insert(s, v.clone()) {
                        Ok(Some(_)) => {
                            let e = REK::HeaderFieldAlreadyExistsError.into_error();
                            return Err(e).map_err_into(REK::HeaderFieldWriteError);
                        },
                        Err(e) => return Err(e).map_err_into(REK::HeaderFieldWriteError),
                        _ => (),
                    }
                }
                &None => {
                    debug!("Not going to insert.");
                }
            }
        }

        Ok(fle)
    }

}

