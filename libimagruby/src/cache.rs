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

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;

lazy_static! {
    /// A cache for FileLockEntry objects.
    ///
    /// # Why do we need this?
    ///
    /// "ruru" does us give the ability to move objects to the Ruby namespace by wrapping them into
    /// a struct that can be handled by the Ruby VM. This is awesome.
    /// Although, we cannot get the Object back into the Rust namespace (at least not as `Object`,
    /// only as `&mut Object`).
    ///
    /// Now, have a look at `Store::save_as()` for example. It has the type signature
    /// `fn save_as(&self, FileLockEntry, StoreId) -> Result<()>;`.
    /// This means that we need to _move_ a `FileLockEntry` into the Store.
    ///
    /// But we cannot, if the FileLockEntry is in the Ruby namespace (we cannot get it back).
    ///
    /// The solution is simple and complex at the same time: Do not move any object into the Ruby
    /// namespace!
    ///
    /// What we do: If the Ruby code wants us to get a `FileLockEntry`, we actually move the
    /// `FileLockEntry` into this `FILE_LOCK_ENTRY_CACHE` and give the Ruby process a `Handle` for
    /// the `FileLockEntry` object.
    ///
    /// From the Ruby world, it looks like a `FileLockEntry`, but it is not. The implementations in
    /// this very library fetch the `FileLockEntry` from this cache and operate on it, putting it
    /// back into this cache afterwards.
    ///
    /// # Performance?
    ///
    /// I don't care right now. It is Ruby, it is slow anyways. If it works, I don't mind. And I
    /// don't see why we couldn't change this implementation later under the hood...
    pub static ref FILE_LOCK_ENTRY_CACHE: Arc<Mutex<BTreeMap<StoreId, FileLockEntry<'static>>>> = {
        Arc::new(Mutex::new(BTreeMap::new()))
    };
}

