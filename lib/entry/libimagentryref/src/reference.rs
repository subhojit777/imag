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

//! The Ref object is a helper over the link functionality, so one is able to create references to
//! files outside of the imag store.

use std::path::PathBuf;
use std::result::Result as RResult;

use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;
use libimagstore::store::Entry;

use toml_query::read::TomlValueReadExt;
use refstore::UniqueRefPathGenerator;

use error::Result;
use error::RefError as RE;
use error::RefErrorKind as REK;

pub trait Ref {

    /// Check whether the underlying object is actually a ref
    fn is_ref(&self) -> Result<bool>;

    /// Get the stored hash.
    ///
    /// Does not need a `UniqueRefPathGenerator` as it reads the hash stored in the header
    fn get_hash(&self) -> Result<String>;

    /// Get the referenced path.
    ///
    /// Does not need a `UniqueRefPathGenerator` as it reads the path stored in the header.
    fn get_path(&self) -> Result<PathBuf>;

    /// Check whether the referenced file still matches its hash
    fn hash_valid<RPG: UniqueRefPathGenerator>(&self) -> Result<bool>;

    /// Update the stored hash
    ///
    /// This updates the hash in the header and moves the entry to the appropriate location
    fn update_hash<RPG: UniqueRefPathGenerator>(&mut self, store: &Store) -> Result<bool>;

    /// Alias for `r.fs_link_exists() && r.deref().is_file()`
    fn is_ref_to_file(&self) -> Result<bool> {
        self.get_path().map(|p| p.is_file())
    }

    /// Alias for `r.fs_link_exists() && r.deref().is_dir()`
    fn is_ref_to_dir(&self) -> Result<bool> {
        self.get_path().map(|p| p.is_dir())
    }

    /// Alias for `!Ref::fs_link_exists()`
    fn is_dangling(&self) -> Result<bool> {
        self.get_path().map(|p| !p.exists())
    }

}

provide_kindflag_path!(pub IsRef, "ref.is_ref");

impl Ref for Entry {

    /// Check whether the underlying object is actually a ref
    fn is_ref(&self) -> Result<bool> {
        self.is::<IsRef>().map_err(From::from)
    }

    fn get_hash(&self) -> Result<String> {
        unimplemented!()
    }

    fn get_path(&self) -> Result<PathBuf> {
        unimplemented!()
    }

    fn hash_valid<RPG: UniqueRefPathGenerator>(&self) -> Result<bool> {
        unimplemented!()
    }

    fn update_hash<RPG: UniqueRefPathGenerator>(&mut self, store: &Store) -> Result<bool> {
        unimplemented!()
    }

}

