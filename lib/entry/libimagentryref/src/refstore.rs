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

use std::path::Path;
use std::path::PathBuf;

use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagstore::storeid::StoreId;

use error::RefError as RE;
use reference::Ref;

/// A UniqueRefPathGenerator generates unique Pathes
///
/// It is basically a functor which generates a StoreId from a &Path.
/// For more information have a look at the documentation of RefStore.
pub trait UniqueRefPathGenerator {
    type Error: From<RE>;

    /// The collection the `StoreId` should be created for
    fn collection() -> &'static str {
        "ref"
    }

    /// A function which should generate a unique string for a Path
    fn unique_hash<A: AsRef<Path>>(path: A) -> Result<String, Self::Error>;

    /// Postprocess the generated `StoreId` object
    fn postprocess_storeid(sid: StoreId) -> Result<StoreId, Self::Error> {
        Ok(sid)
    }
}

/// A extensions for the `Store` to handle `Ref` objects
///
/// The RefStore handles refs using a `UniqueRefPathGenerator`. The `UniqueRefPathGenerator`, as it
/// name suggests, generates unique `StoreId`s for a `&Path`. It is a functor `&Path -> StoreId`.
///
/// It provides three functions which are called in the following sequence:
///
/// * The `UniqueRefPathGenerator::collection()` function is used for get the collection a `StoreId`
///   should be in (The first element of the `StoreId` path)
/// * The `UniqueRefPathGenerator::unique_hash()` gets the `&Path` which it then should generate a
///   unique String for. How this is done does not matter. It can hash the Path itself, read the
///   file and hash that or something else. It should be reproduceable, though.
/// * These two parts are joined and put into a `StoreId` which the
///   `UniqueRefPathGenerator::postprocess_storeid()` function is then allowed to postprocess (for
///   example add more parts to the StoreId). The default implementation does nothing.
///
/// The StoreId which is generated is then used to carry out the actual action (reading, creating
/// ...).
/// If a entry is created, header information is set (that it is a ref, the hash which was just
/// generated and the path of the referenced file)
///
/// # Details
///
/// The `UniqueRefPathGenerator` is passed as type parameter to enforce some semantics:
///
/// * The used `UniqueRefPathGenerator` is defined by the implementation rather than by the runtime
///   of the program or some environment. Of course this is only a small hurdle to enforce this, but
///   a hint.
/// * The `UniqueRefPathGenerator` is a functor which does not carry state.
///
pub trait RefStore<'a> {

    fn get_ref<RPG: UniqueRefPathGenerator, H: AsRef<str>>(&'a self, hash: H) -> Result<Option<FileLockEntry<'a>>, RPG::Error>;
    fn create_ref<RPG: UniqueRefPathGenerator, A: AsRef<Path>>(&'a self, path: A) -> Result<FileLockEntry<'a>, RPG::Error>;
    fn retrieve_ref<RPG: UniqueRefPathGenerator, A: AsRef<Path>>(&'a self, path: A) -> Result<FileLockEntry<'a>, RPG::Error>;

}

impl<'a> RefStore<'a> for Store {

    fn get_ref<RPG: UniqueRefPathGenerator, H: AsRef<str>>(&'a self, hash: H)
        -> Result<Option<FileLockEntry<'a>>, RPG::Error>
    {
        let sid = StoreId::new_baseless(PathBuf::from(format!("{}/{}", RPG::collection(), hash.as_ref())))
            .map_err(RE::from)?;

        debug!("Getting: {:?}", sid);
        self.get(sid)
            .map_err(RE::from)
            .map_err(RPG::Error::from)
    }

    fn create_ref<RPG: UniqueRefPathGenerator, A: AsRef<Path>>(&'a self, path: A)
        -> Result<FileLockEntry<'a>, RPG::Error>
    {
        let filepath = path.as_ref().to_path_buf();
        let hash     = RPG::unique_hash(path)?;
        let pathbuf  = PathBuf::from(format!("{}/{}", RPG::collection(), hash));
        let sid      = StoreId::new_baseless(pathbuf.clone()).map_err(RE::from)?;

        debug!("Creating: {:?}", sid);
        self.create(sid)
            .map_err(RE::from)
            .and_then(|mut fle| {
                fle.make_ref(hash, filepath)?;
                Ok(fle)
            })
            .map_err(RPG::Error::from)
    }

    fn retrieve_ref<RPG: UniqueRefPathGenerator, A: AsRef<Path>>(&'a self, path: A)
        -> Result<FileLockEntry<'a>, RPG::Error>
    {
        match self.get_ref::<RPG, String>(RPG::unique_hash(path.as_ref())?)? {
            Some(r) => Ok(r),
            None    => self.create_ref::<RPG, A>(path),
        }
    }

}

