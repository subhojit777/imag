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

use libimagstore::store::Store;
use libimagerror::trace::trace_error;
use std::error::Error;
use std::ops::Deref;
use std::ops::DerefMut;

use ruru::{Class, Object, AnyObject, Boolean, RString, VM, Hash, NilClass, VerifiedObject};

use ruby_utils::IntoToml;
use toml_utils::IntoRuby;
use util::Wrap;
use util::Unwrap;

use storeid::RStoreId;
use entry::RFileLockEntry;
use cache::StoreHandle;

wrappable_struct!(StoreHandle, StoreWrapper, STORE_WRAPPER);
class!(RStore);
impl_wrap!(StoreHandle, STORE_WRAPPER);
impl_unwrap!(RStore, StoreHandle, STORE_WRAPPER);
impl_verified_object!(RStore);

macro_rules! call_on_store_by_handle {
    ($store_handle:ident -> $name:ident -> $operation:block) => {{
        call_on_store_by_handle!($store_handle -> $name -> $operation on fail return NilClass::new().to_any_object())
    }};

    ($store_handle:ident -> $name:ident -> $operation:block on fail return $ex:expr) => {{
        use cache::RUBY_STORE_CACHE;

        let arc = RUBY_STORE_CACHE.clone();
        {
            let lock = arc.lock();
            match lock {
                Ok(mut hm) => {
                    match hm.get($store_handle) {
                        Some($name) => { $operation },
                        None => {
                            VM::raise(Class::from_existing("RuntimeError"),
                                    "Tried to operate on non-existing object");
                            $ex
                        }
                    }
                },
                Err(e) => {
                    VM::raise(Class::from_existing("RuntimeError"), e.description());
                    $ex
                }
            }
        }
    }};
}

macro_rules! call_on_store {
    ($itself:ident ($wrapper:ident) -> $name:ident fetch $fle_handle_name:ident -> $fle_name:ident -> $operation:block) => {{
        call_on_store!($itself ($wrapper) -> $name fetch $fle_handle_name -> $fle_name -> $operation on fail return NilClass::new())
    }};
    ($itself:ident ($wrapper:ident) -> $name:ident fetch $fle_handle_name:ident -> $fle_name:ident -> $operation:block on fail return $ex:expr) => {{
        let handle = $itself.get_data(&*$wrapper);
        call_on_store_by_handle!(handle -> $name -> {
            let $fle_name = match $name.get($fle_handle_name) {
                Ok(Some(fle)) => fle,
                Ok(None) => {
                    VM::raise(Class::from_existing("RuntimeError"), "Obj does not exist");
                    return $ex
                },
                Err(e) => {
                    VM::raise(Class::from_existing("RuntimeError"), e.description());
                    return $ex
                },
            };
            $operation
        } on fail return $ex)
    }};

    ($itself:ident ($wrapper:ident) -> $name:ident -> $operation:block on fail return $ex:expr) => {{
        let handle = $itself.get_data(&*$wrapper);
        call_on_store_by_handle!(handle -> $name -> $operation on fail return $ex)
    }};
    ($itself:ident ($wrapper:ident) -> $name:ident -> $operation:block) => {{
        let handle = $itself.get_data(&*$wrapper);
        call_on_store_by_handle!(handle -> $name -> $operation)
    }};
}

methods!(
    RStore,
    itself,

    // Create an FileLockEntry in the store
    //
    // # Returns:
    //
    // On success: A RFileLockEntry
    // On failure: Nil
    // On error: Nil + Exception
    //
    fn create(id: RStoreId) -> AnyObject {
        unimplemented!()
    }

    // Retrieve an FileLockEntry from the store
    //
    // # Returns:
    //
    // On success: A RFileLockEntry
    // On error: Nil + Exception
    //
    fn retrieve(id: RStoreId) -> AnyObject {
        unimplemented!()
    }

    // Get an FileLockEntry from the store
    //
    // # Returns:
    //
    // On success, if there is some: A RFileLockEntry
    // On success, if there is none: Nil
    // On error: Nil + Exception
    //
    fn get(sid: RStoreId) -> AnyObject {
        unimplemented!()
    }

    // Get all FileLockEntry of a module from the store
    //
    // # Returns:
    //
    // On success: A Array[RFileLockEntry]
    // On error: Nil + Exception
    //
    fn retrieve_for_module(name: RString) -> AnyObject {
        unimplemented!()
    }

    // Update a FileLockEntry in the store
    //
    // # Returns:
    //
    // On success: Nil
    // On error: Nil + Exception
    //
    fn update(fle: RFileLockEntry) -> NilClass {
        unimplemented!()
    }

    // Delete a FileLockEntry from the store
    //
    // # Returns:
    //
    // On success: Nil
    // On error: Nil + Exception
    //
    fn delete(sid: RStoreId) -> NilClass {
        unimplemented!()
    }

    // Save a FileLockEntry in a new path inside the store, keep the RFileLockEntry
    //
    // # Returns:
    //
    // On success: Nil
    // On error: Nil + Exception
    //
    fn save_to(fle: RFileLockEntry, sid: RStoreId) -> NilClass {
        let fle = typecheck!(fle).unwrap().clone();
        let sid = typecheck!(sid).unwrap().clone();

        call_on_store!(itself (STORE_WRAPPER) -> store fetch fle -> real_fle -> {
            if let Err(e) = store.save_to(&real_fle, sid) {
                trace_error(&e);
                VM::raise(Class::from_existing("RuntimeError"), e.description());
            }
            NilClass::new()
        } on fail return NilClass::new())
    }

    // Save a FileLockEntry in a new path inside the store, move the RFileLockEntry
    //
    // # Returns:
    //
    // On success: Nil
    // On error: Nil + Exception
    //
    fn save_as(fle: RFileLockEntry, sid: RStoreId) -> NilClass {
        let fle = typecheck!(fle).unwrap().clone();
        let sid = typecheck!(sid).unwrap().clone();

        call_on_store!(itself (STORE_WRAPPER) -> store fetch fle -> real_fle -> {
            if let Err(e) = store.save_as(real_fle, sid) {
                trace_error(&e);
                VM::raise(Class::from_existing("RuntimeError"), e.description());
            }
            NilClass::new()
        } on fail return NilClass::new())
    }

    // Move one entry in the store to another place, by its ID
    //
    // # Returns:
    //
    // On success: Nil
    // On error: Nil + Exception
    //
    fn move_by_id(old: RStoreId, nw: RStoreId) -> NilClass {
        let old = typecheck!(old).unwrap().clone();
        let nw  = typecheck!(nw).unwrap().clone();

        call_on_store!(itself (STORE_WRAPPER) -> store -> {
            if let Err(e) = store.move_by_id(old, nw) {
                trace_error(&e);
                VM::raise(Class::from_existing("RuntimeError"), e.description());
            }
            NilClass::new()
        } on fail return NilClass::new())
    }

    // Get the path of the store object
    //
    // # Returns:
    //
    // A RString
    //
    fn path() -> RString {
        call_on_store!(itself (STORE_WRAPPER) -> store -> {
            store.path()
                .clone()
                .to_str()
                .map(RString::new)
                .unwrap_or(RString::new(""))
        } on fail return RString::new(""))
    }

);

pub fn setup() {
    unimplemented!()
}
