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

#[allow(unused_variables)]

use libimagstore::store::Store;
use libimagerror::trace::trace_error;
use std::error::Error;

use ruru::{Class, Object, AnyObject, Boolean, RString, VM, NilClass, VerifiedObject};

use util::Wrap;
use util::Unwrap;

use storeid::RStoreId;
use entry::RFileLockEntryHandle;
use cache::StoreHandle;

wrappable_struct!(StoreHandle, StoreWrapper, STORE_WRAPPER);
class!(RStoreHandle);
impl_wrap!(StoreHandle => STORE_WRAPPER);
impl_unwrap!(RStoreHandle => StoreHandle => STORE_WRAPPER);
impl_verified_object!(RStoreHandle);

macro_rules! call_on_store_by_handle {
    {
        $store_handle:ident named $name:ident inside $operation:block
    }=> {{
        call_on_store_by_handle! {
            $store_handle
            named $name
            inside $operation
            on fail return NilClass::new().to_any_object()
        }
    }};

    {
        $store_handle:ident named $name:ident inside $operation:block on fail return $ex:expr
    } => {{
        use cache::RUBY_STORE_CACHE;

        let arc = RUBY_STORE_CACHE.clone();
        {
            let lock = arc.lock();
            match lock {
                Ok(mut hm) => {
                    match hm.get($store_handle) {
                        Some($name) => { $operation },
                        None => {
                            VM::raise(Class::from_existing("RImagStoreReadError"),
                                    "Tried to operate on non-existing object");
                            $ex
                        }
                    }
                },
                Err(e) => {
                    VM::raise(Class::from_existing("RImagError"), e.description());
                    $ex
                }
            }
        }
    }};
}

macro_rules! call_on_store {
    {
        $store_name:ident <- $itself:ident wrapped inside $wrapper:ident,
        $fle_name:ident <- fetch $fle_handle_name:ident
        operation $operation:block
    } => {
        call_on_store! {
            $store_name <- $itself wrapped inside $wrapper,
            $fle_name <- fetch $fle_handle_name,
            operation $operation,
            on fail return NilClass::new()
        }
    };

    {
        $store_name:ident <- $itself:ident wrapped inside $wrapper:ident,
        $fle_name:ident <- fetch $fle_handle_name:ident,
        operation $operation:block,
        on fail return $fail_expr:expr
    } => {
        let handle = $itself.get_data(&*$wrapper);
        call_on_store_by_handle! {
            handle named $store_name inside {
                let $fle_name = match $store_name.get($fle_handle_name) {
                    Ok(Some(fle)) => fle,
                    Ok(None) => {
                        VM::raise(Class::from_existing("RImagStoreReadError"), "Obj does not exist");
                        return $fail_expr
                    },
                    Err(e) => {
                        VM::raise(Class::from_existing("RImagStoreReadError"), e.description());
                        return $fail_expr
                    },
                };
                $operation
            }
            on fail return $fail_expr
        }
    };

    {
        $store_name:ident <- $itself:ident wrapped inside $wrapper:ident,
        operation $operation:block,
        on fail return $fail_expr:expr
    } => {
        let handle = $itself.get_data(&*$wrapper);
        call_on_store_by_handle! {
            handle named $store_name inside $operation on fail return $fail_expr
        }
    };

    {
        $store_name:ident <- $itself:ident wrapped inside $wrapper:ident,
        operation $block
    } => {
        let handle = $itself.get_data(&*$wrapper);
        call_on_store_by_handle! { handle named $name inside $operation }
    };
}

methods!(
    RStoreHandle,
    itself,

    // Build a new Store object, return a handle to it.
    //
    // This function takes a boolean whether the store should include debugging functionality
    // (namingly the debug hooks) and a runtimepath, where the store lifes.
    // It then builds a Store object (raising errors on failure and returning Nil) and a handle for
    // it.
    // It puts the store object and the handle in the cache and returns the handle as object to the
    // Ruby code.
    //
    // # Returns
    //
    // Nil on failure (including raising an error)
    // StoreHandle on success
    //
    fn new(store_debugging: Boolean, rtp: RString) -> AnyObject {
        use std::path::PathBuf;
        use libimagerror::trace::trace_error;
        use libimagerror::trace::trace_error_dbg;
        use libimagrt::configuration::ConfigErrorKind;
        use libimagrt::configuration::Configuration;
        use libimagstore::error::StoreErrorKind;
        use libimagstore::hook::Hook;
        use libimagstore::hook::position::HookPosition as HP;
        use libimagstorestdhook::debug::DebugHook;
        use libimagstorestdhook::vcs::git::delete::DeleteHook as GitDeleteHook;
        use libimagstorestdhook::vcs::git::store_unload::StoreUnloadHook as GitStoreUnloadHook;
        use libimagstorestdhook::vcs::git::update::UpdateHook as GitUpdateHook;

        use cache::RUBY_STORE_CACHE;

        let store_debugging = typecheck!(store_debugging or return any NilClass::new()).to_bool();
        let rtp = PathBuf::from(typecheck!(rtp or return any NilClass::new()).to_string());

        if !rtp.exists() || !rtp.is_dir() {
            VM::raise(Class::from_existing("RImagError"), "Runtimepath not a directory");
            return NilClass::new().to_any_object();
        }

        let store_config = match Configuration::new(&rtp) {
            Ok(mut cfg) => cfg.store_config().cloned(),
            Err(e) => if e.err_type() != ConfigErrorKind::NoConfigFileFound {
                VM::raise(Class::from_existing("RImagError"), e.description());
                return NilClass::new().to_any_object();
            } else {
                warn!("No config file found.");
                warn!("Continuing without configuration file");
                None
            },
        };

        let storepath = {
            let mut spath = rtp.clone();
            spath.push("store");
            spath
        };

        let store = Store::new(storepath.clone(), store_config).map(|mut store| {
            // If we are debugging, generate hooks for all positions
            if store_debugging {
                let hooks : Vec<(Box<Hook>, &str, HP)> = vec![
                    (Box::new(DebugHook::new(HP::PreCreate))          , "debug", HP::PreCreate),
                    (Box::new(DebugHook::new(HP::PostCreate))         , "debug", HP::PostCreate),
                    (Box::new(DebugHook::new(HP::PreRetrieve))        , "debug", HP::PreRetrieve),
                    (Box::new(DebugHook::new(HP::PostRetrieve))       , "debug", HP::PostRetrieve),
                    (Box::new(DebugHook::new(HP::PreUpdate))          , "debug", HP::PreUpdate),
                    (Box::new(DebugHook::new(HP::PostUpdate))         , "debug", HP::PostUpdate),
                    (Box::new(DebugHook::new(HP::PreDelete))          , "debug", HP::PreDelete),
                    (Box::new(DebugHook::new(HP::PostDelete))         , "debug", HP::PostDelete),
                ];

                // If hook registration fails, trace the error and warn, but continue.
                for (hook, aspectname, position) in hooks {
                    if let Err(e) = store.register_hook(position, &String::from(aspectname), hook) {
                        if e.err_type() == StoreErrorKind::HookRegisterError {
                            trace_error_dbg(&e);
                            warn!("Registering debug hook with store failed");
                        } else {
                            trace_error(&e);
                        };
                    }
                }
            }

            let sp = storepath;

            let hooks : Vec<(Box<Hook>, &str, HP)> = vec![
                (Box::new(GitDeleteHook::new(sp.clone(), HP::PostDelete)), "vcs", HP::PostDelete),
                (Box::new(GitUpdateHook::new(sp.clone(), HP::PostUpdate)), "vcs", HP::PostUpdate),
                (Box::new(GitStoreUnloadHook::new(sp)),                    "vcs", HP::StoreUnload),
            ];

            for (hook, aspectname, position) in hooks {
                if let Err(e) = store.register_hook(position, &String::from(aspectname), hook) {
                    if e.err_type() == StoreErrorKind::HookRegisterError {
                        trace_error_dbg(&e);
                        warn!("Registering git hook with store failed");
                    } else {
                        trace_error(&e);
                    };
                }
            }

            store
        });

        let store = match store {
            Ok(s) => s,
            Err(e) => {
                VM::raise(Class::from_existing("RImagStoreError"), e.description());
                return NilClass::new().to_any_object();
            },
        };

        let store_handle = StoreHandle::new();

        let arc = RUBY_STORE_CACHE.clone();
        {
            let lock = arc.lock();
            match lock {
                Ok(mut hm) => {
                    hm.insert(store_handle.clone(), store);
                    return store_handle.wrap().to_any_object();
                },
                Err(e) => {
                    VM::raise(Class::from_existing("RImagError"), e.description());
                    return NilClass::new().to_any_object();
                }
            }
        }

    }


    // Create an FileLockEntry in the store
    //
    // # Returns:
    //
    // On success: A RFileLockEntry
    // On failure: Nil
    // On error: Nil + Exception
    //
    fn create(id: RStoreId) -> AnyObject {
        use entry::FileLockEntryHandle;
        let sid = typecheck!(id or return any NilClass::new()).unwrap().clone();

        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            operation {
                match store.create(sid.clone()) {
                    Err(e) => {
                        trace_error(&e);
                        VM::raise(Class::from_existing("RImagStoreWriteError"), e.description());
                        NilClass::new().to_any_object()
                    },
                    Ok(entry) => {
                        // Take the location (StoreId) of the entry (we know it exists... so this
                        // is fine) and wrap it into a RFileLockEntry which is then returned to the
                        // user (as handle)
                        let sid = entry.get_location().clone();
                        let store_handle = itself.get_data(&*STORE_WRAPPER).clone();
                        FileLockEntryHandle::new(store_handle, sid).wrap()
                    },
                }
            },
            on fail return NilClass::new().to_any_object()
        }
    }

    // Retrieve an FileLockEntry from the store
    //
    // # Returns:
    //
    // On success: A RFileLockEntry
    // On error: Nil + Exception
    //
    fn retrieve(id: RStoreId) -> AnyObject {
        use entry::FileLockEntryHandle;
        let sid = typecheck!(id or return any NilClass::new()).unwrap().clone();

        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            operation {
                match store.retrieve(sid.clone()) {
                    Err(e) => {
                        trace_error(&e);
                        VM::raise(Class::from_existing("RImagStoreWriteError"), e.description());
                        NilClass::new().to_any_object()
                    },
                    Ok(entry) => {
                        // Take the location (StoreId) of the entry (we know it exists... so this
                        // is fine) and wrap it into a RFileLockEntry which is then returned to the
                        // user (as handle)
                        let sid = entry.get_location().clone();
                        let store_handle = itself.get_data(&*STORE_WRAPPER).clone();
                        FileLockEntryHandle::new(store_handle, sid).wrap()
                    },
                }
            },
            on fail return NilClass::new().to_any_object()
        }
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
        use entry::FileLockEntryHandle;
        let sid = typecheck!(sid or return any NilClass::new()).unwrap().clone();

        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            operation {
                match store.get(sid.clone()) {
                    Err(e) => {
                        trace_error(&e);
                        VM::raise(Class::from_existing("RImagStoreWriteError"), e.description());
                        NilClass::new().to_any_object()
                    },
                    Ok(None) => NilClass::new().to_any_object(),
                    Ok(Some(entry)) => {
                        // Take the location (StoreId) of the entry (we know it exists... so this
                        // is fine) and wrap it into a RFileLockEntry which is then returned to the
                        // user (as handle)
                        let sid = entry.get_location().clone();
                        let store_handle = itself.get_data(&*STORE_WRAPPER).clone();
                        FileLockEntryHandle::new(store_handle, sid).wrap()
                    },
                }
            },
            on fail return NilClass::new().to_any_object()
        }
    }

    // Get all FileLockEntry of a module from the store
    //
    // # Returns:
    //
    // On success: A Array[RFileLockEntry]
    // On error: Nil + Exception
    //
    fn retrieve_for_module(name: RString) -> AnyObject {
        use entry::FileLockEntryHandle as FLEH;
        use ruru::Array;

        let name = typecheck!(name or return any NilClass::new()).to_string();

        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            operation {
                match store.retrieve_for_module(&name) {
                    Err(e) => {
                        trace_error(&e);
                        VM::raise(Class::from_existing("RImagStoreWriteError"), e.description());
                        NilClass::new().to_any_object()
                    },
                    Ok(iter) => {
                        let store_handle = itself.get_data(&*STORE_WRAPPER).clone();
                        iter.map(|sid| FLEH::new(store_handle.clone(), sid).wrap())
                            .fold(Array::new(), |mut a, e| a.push(e))
                            .to_any_object()
                    },
                }
            },
            on fail return NilClass::new().to_any_object()
        }
    }

    // Update a FileLockEntry in the store
    //
    // # Returns:
    //
    // On success: Nil
    // On error: Nil + Exception
    //
    fn update(fle: RFileLockEntryHandle) -> NilClass {
        let fle = typecheck!(fle).unwrap().fle_handle().clone();

        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            real_fle <- fetch fle,
            operation {
                let mut real_fle = real_fle; // rebind for mut
                if let Err(e) = store.update(&mut real_fle) {
                    trace_error(&e);
                    VM::raise(Class::from_existing("RImagStoreWriteError"), e.description());
                }
                NilClass::new()
            },
            on fail return NilClass::new()
        }
    }

    // Delete a FileLockEntry from the store
    //
    // # Returns:
    //
    // On success: Nil
    // On error: Nil + Exception
    //
    fn delete(sid: RStoreId) -> NilClass {
        let sid = typecheck!(sid).unwrap().clone();

        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            operation {
                if let Err(e) = store.delete(sid) {
                    trace_error(&e);
                    VM::raise(Class::from_existing("RImagStoreWriteError"), e.description());
                }
                NilClass::new()
            },
            on fail return NilClass::new()
        }
    }

    // Save a FileLockEntry in a new path inside the store, keep the RFileLockEntry
    //
    // # Returns:
    //
    // On success: Nil
    // On error: Nil + Exception
    //
    fn save_to(fle: RFileLockEntryHandle, sid: RStoreId) -> NilClass {
        let fle = typecheck!(fle).unwrap().fle_handle().clone();
        let sid = typecheck!(sid).unwrap().clone();

        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            real_fle <- fetch fle,
            operation {
                if let Err(e) = store.save_to(&real_fle, sid) {
                    trace_error(&e);
                    VM::raise(Class::from_existing("RImagStoreWriteError"), e.description());
                }
                NilClass::new()
            },
            on fail return NilClass::new()
        }
    }

    // Save a FileLockEntry in a new path inside the store, move the RFileLockEntry
    //
    // # Returns:
    //
    // On success: Nil
    // On error: Nil + Exception
    //
    fn save_as(fle: RFileLockEntryHandle, sid: RStoreId) -> NilClass {
        let fle = typecheck!(fle).unwrap().fle_handle().clone();
        let sid = typecheck!(sid).unwrap().clone();

        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            real_fle <- fetch fle,
            operation {
                if let Err(e) = store.save_as(real_fle, sid) {
                    trace_error(&e);
                    VM::raise(Class::from_existing("RImagStoreWriteError"), e.description());
                }
                NilClass::new()
            },
            on fail return NilClass::new()
        }
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

        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            operation {
                if let Err(e) = store.move_by_id(old, nw) {
                    trace_error(&e);
                    VM::raise(Class::from_existing("RImagStoreWriteError"), e.description());
                }
                NilClass::new()
            },
            on fail return NilClass::new()
        }
    }

    // Get the path of the store object
    //
    // # Returns:
    //
    // A RString
    //
    fn path() -> RString {
        call_on_store! {
            store <- itself wrapped inside STORE_WRAPPER,
            operation {
                store.path()
                    .clone()
                    .to_str()
                    .map(RString::new)
                    .unwrap_or(RString::new(""))
            },
            on fail return RString::new("")
        }
    }

);

pub fn setup() -> Class {
    let mut class = Class::new("RStoreHandle", None);
    class.define(|itself| {
        itself.def_self("new"            , new);
        itself.def("create"              , create);
        itself.def("retrieve"            , retrieve);
        itself.def("get"                 , get);
        itself.def("retrieve_for_module" , retrieve_for_module);
        itself.def("update"              , update);
        itself.def("delete"              , delete);
        itself.def("save_to"             , save_to);
        itself.def("save_as"             , save_as);
        itself.def("move_by_id"          , move_by_id);
        itself.def("path"                , path);
    });
    class
}

