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

//! The Ref object is a helper over the link functionality, so one is able to create references to
//! files outside of the imag store.

use std::path::PathBuf;
use std::fs::File;
use std::fs::Permissions;

use libimagstore::store::Entry;

use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::set::TomlValueSetExt;

use error::RefErrorKind as REK;
use error::RefError as RE;
use error::ResultExt;
use error::Result;
use hasher::*;

pub trait Ref {

    /// Get the hash from the path of the ref
    fn get_path_hash(&self) -> Result<String>;

    /// Get the hash of the link target which is stored in the ref object
    fn get_stored_hash(&self) -> Result<String>;

    /// Get the hahs of the link target which is stored in the ref object, which is hashed with a
    /// custom Hasher instance.
    fn get_stored_hash_with_hasher<H: Hasher>(&self, h: &H) -> Result<String>;

    /// Get the hash of the link target by reading the link target and hashing the contents
    fn get_current_hash(&self) -> Result<String>;

    /// Get the hash of the link target by reading the link target and hashing the contents with the
    /// custom hasher
    fn get_current_hash_with_hasher<H: Hasher>(&self, h: H) -> Result<String>;

    /// check whether the pointer the Ref represents still points to a file which exists
    fn fs_link_exists(&self) -> Result<bool>;

    /// Alias for `r.fs_link_exists() && r.deref().is_file()`
    fn is_ref_to_file(&self) -> Result<bool>;

    /// Alias for `r.fs_link_exists() && r.deref().is_dir()`
    fn is_ref_to_dir(&self) -> Result<bool>;

    /// Alias for `!Ref::fs_link_exists()`
    fn is_dangling(&self) -> Result<bool>;

    /// check whether the pointer the Ref represents is valid
    /// This includes:
    ///     - Hashsum of the file is still the same as stored in the Ref
    ///     - file permissions are still valid
    fn fs_link_valid(&self) -> Result<bool>;

    /// Check whether the file permissions of the referenced file are equal to the stored
    /// permissions
    fn fs_link_valid_permissions(&self) -> Result<bool>;

    /// Check whether the Hashsum of the referenced file is equal to the stored hashsum
    fn fs_link_valid_hash(&self) -> Result<bool>;

    /// Update the Ref by re-checking the file from FS
    /// This errors if the file is not present or cannot be read()
    fn update_ref(&mut self) -> Result<()>;

    /// Update the Ref by re-checking the file from FS using the passed Hasher instance
    /// This errors if the file is not present or cannot be read()
    fn update_ref_with_hasher<H: Hasher>(&mut self, h: &H) -> Result<()>;

    /// Get the path of the file which is reffered to by this Ref
    fn fs_file(&self) -> Result<PathBuf>;

    /// Re-find a referenced file
    ///
    /// This function tries to re-find a ref by searching all directories in `search_roots` recursively
    /// for a file which matches the hash of the Ref.
    ///
    /// If `search_roots` is `None`, it starts at the filesystem root `/`.
    ///
    /// If the target cannot be found, this yields a RefTargetDoesNotExist error kind.
    ///
    /// # Warning
    ///
    /// This option causes heavy I/O as it recursively searches the Filesystem.
    fn refind(&self, search_roots: Option<Vec<PathBuf>>) -> Result<PathBuf>;

    /// See documentation of `Ref::refind()`
    fn refind_with_hasher<H: Hasher>(&self, search_roots: Option<Vec<PathBuf>>, h: H)
        -> Result<PathBuf>;

    /// Get the permissions of the file which are present
    fn get_current_permissions(&self) -> Result<Permissions>;
}


impl Ref for Entry {

    /// Get the hash from the path of the ref
    fn get_path_hash(&self) -> Result<String> {
        self.get_location()
            .clone()
            .into_pathbuf()
            .map_err(From::from)
            .and_then(|pb| {
                pb.file_name()
                    .and_then(|osstr| osstr.to_str())
                    .and_then(|s| s.split("~").next())
                    .map(String::from)
                    .ok_or(String::from("String splitting error"))
                    .map_err(From::from)
            })
    }

    /// Get the hash of the link target which is stored in the ref object
    fn get_stored_hash(&self) -> Result<String> {
        self.get_stored_hash_with_hasher(&DefaultHasher::new())
    }

    /// Get the hahs of the link target which is stored in the ref object, which is hashed with a
    /// custom Hasher instance.
    fn get_stored_hash_with_hasher<H: Hasher>(&self, h: &H) -> Result<String> {
        self.get_header()
            .read(&format!("ref.content_hash.{}", h.hash_name())[..])?
            .ok_or(RE::from_kind(REK::HeaderFieldMissingError))?
            .as_str()
            .map(String::from)
            .ok_or(RE::from_kind(REK::HeaderTypeError))
    }

    /// Get the hash of the link target by reading the link target and hashing the contents
    fn get_current_hash(&self) -> Result<String> {
        self.get_current_hash_with_hasher(DefaultHasher::new())
    }

    /// Get the hash of the link target by reading the link target and hashing the contents with the
    /// custom hasher
    fn get_current_hash_with_hasher<H: Hasher>(&self, mut h: H) -> Result<String> {
        self.fs_file()
            .and_then(|pb| File::open(pb.clone()).map(|f| (pb, f)).map_err(From::from))
            .and_then(|(path, mut file)| h.create_hash(&path, &mut file))
    }

    /// check whether the pointer the Ref represents still points to a file which exists
    fn fs_link_exists(&self) -> Result<bool> {
        self.fs_file().map(|pathbuf| pathbuf.exists())
    }

    /// Alias for `r.fs_link_exists() && r.deref().is_file()`
    fn is_ref_to_file(&self) -> Result<bool> {
        self.fs_file().map(|pathbuf| pathbuf.is_file())
    }

    /// Alias for `r.fs_link_exists() && r.deref().is_dir()`
    fn is_ref_to_dir(&self) -> Result<bool> {
        self.fs_file().map(|pathbuf| pathbuf.is_dir())
    }

    /// Alias for `!Ref::fs_link_exists()`
    fn is_dangling(&self) -> Result<bool> {
        self.fs_link_exists().map(|b| !b)
    }

    /// check whether the pointer the Ref represents is valid
    /// This includes:
    ///     - Hashsum of the file is still the same as stored in the Ref
    ///     - file permissions are still valid
    fn fs_link_valid(&self) -> Result<bool> {
        match (self.fs_link_valid_permissions(), self.fs_link_valid_hash()) {
            (Ok(true) , Ok(true)) => Ok(true),
            (Ok(_)    , Ok(_))    => Ok(false),
            (Err(e)   , _)        => Err(e),
            (_        , Err(e))   => Err(e),
        }
    }

    /// Check whether the file permissions of the referenced file are equal to the stored
    /// permissions
    fn fs_link_valid_permissions(&self) -> Result<bool> {
        self
            .get_header()
            .read("ref.permissions.ro")
            .chain_err(|| REK::HeaderFieldReadError)
            .and_then(|ro| {
                ro.ok_or(RE::from_kind(REK::HeaderFieldMissingError))?
                    .as_bool()
                    .ok_or(RE::from_kind(REK::HeaderTypeError))
            })
            .and_then(|ro| self.get_current_permissions().map(|perm| ro == perm.readonly()))
            .chain_err(|| REK::RefTargetCannotReadPermissions)
    }

    /// Check whether the Hashsum of the referenced file is equal to the stored hashsum
    fn fs_link_valid_hash(&self) -> Result<bool> {
        let stored_hash  = self.get_stored_hash()?;
        let current_hash = self.get_current_hash()?;
        Ok(stored_hash == current_hash)
    }

    /// Update the Ref by re-checking the file from FS
    /// This errors if the file is not present or cannot be read()
    fn update_ref(&mut self) -> Result<()> {
        self.update_ref_with_hasher(&DefaultHasher::new())
    }

    /// Update the Ref by re-checking the file from FS using the passed Hasher instance
    /// This errors if the file is not present or cannot be read()
    fn update_ref_with_hasher<H: Hasher>(&mut self, h: &H) -> Result<()> {
        let current_hash = self.get_current_hash()?; // uses the default hasher
        let current_perm = self.get_current_permissions()?;

        self
            .get_header_mut()
            .set("ref.permissions.ro", Value::Boolean(current_perm.readonly()))
        ?;

        self
            .get_header_mut()
            .set(&format!("ref.content_hash.{}", h.hash_name())[..], Value::String(current_hash))
        ?;

        Ok(())
    }

    /// Get the path of the file which is reffered to by this Ref
    fn fs_file(&self) -> Result<PathBuf> {
        self.get_header()
            .read("ref.path")?
            .ok_or(RE::from_kind(REK::HeaderFieldMissingError))?
            .as_str()
            .map(PathBuf::from)
            .ok_or(RE::from_kind(REK::HeaderTypeError))
    }

    /// Re-find a referenced file
    ///
    /// This function tries to re-find a ref by searching all directories in `search_roots` recursively
    /// for a file which matches the hash of the Ref.
    ///
    /// If `search_roots` is `None`, it starts at the filesystem root `/`.
    ///
    /// If the target cannot be found, this yields a RefTargetDoesNotExist error kind.
    ///
    /// # Warning
    ///
    /// This option causes heavy I/O as it recursively searches the Filesystem.
    fn refind(&self, search_roots: Option<Vec<PathBuf>>) -> Result<PathBuf> {
        self.refind_with_hasher(search_roots, DefaultHasher::new())
    }

    /// See documentation of `Ref::refind()`
    fn refind_with_hasher<H: Hasher>(&self, search_roots: Option<Vec<PathBuf>>, mut h: H)
        -> Result<PathBuf>
    {
        use itertools::Itertools;
        use walkdir::WalkDir;

        self.get_stored_hash()
            .and_then(|stored_hash| {
                search_roots
                    .unwrap_or(vec![PathBuf::from("/")])
                    .into_iter()
                    .map(|root| {
                        WalkDir::new(root)
                            .follow_links(false)
                            .into_iter()
                            .map(|entry| {
                                entry
                                    .map_err(From::from)
                                    .and_then(|entry| {
                                        let pb = PathBuf::from(entry.path());
                                        File::open(entry.path())
                                            .map(|f| (pb, f))
                                            .map_err(From::from)
                                    })
                                    .and_then(|(p, mut f)| {
                                        h.create_hash(&p, &mut f)
                                            .map(|h| (p, h))
                                            .map_err(From::from)
                                    })
                                    .map(|(path, hash)| {
                                        if hash == stored_hash {
                                            Some(path)
                                        } else {
                                            None
                                        }
                                    })
                            })
                            .filter_map(Result::ok)
                            .filter_map(|e| e)
                            .next()
                    })
                    .flatten()
                    .next()
                    .ok_or(RE::from_kind(REK::RefTargetDoesNotExist))
            })
    }

    /// Get the permissions of the file which are present
    fn get_current_permissions(&self) -> Result<Permissions> {
        self.fs_file()
            .and_then(|pb| {
                File::open(pb)
                    .chain_err(|| REK::HeaderFieldReadError)
            })
            .and_then(|file| {
                file
                    .metadata()
                    .map(|md| md.permissions())
                    .chain_err(|| REK::RefTargetCannotReadPermissions)
            })
    }

}

