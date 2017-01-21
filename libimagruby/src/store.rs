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

use ruru::AnyObject;
trait Wrap {
    fn wrap(self) -> AnyObject;
}

macro_rules! impl_wrap {
    ($target: ty, $wrapper: path) => {
        impl Wrap for $target {
            fn wrap(self) -> AnyObject {
                Class::from_existing(concat!("R", stringify!($target)))
                    .wrap_data(self, &*($wrapper))
            }
        }
    }
}


trait Unwrap {
    type Target;
    fn unwrap<'a>(&'a self) -> &'a mut Self::Target;
}

macro_rules! impl_unwrap {
    ($from: ty, $to: ty, $wrapper: path) => {
        impl Unwrap for $from {
            type Target = $to;
            fn unwrap<'a>(&'a self) -> &'a mut $to {
                self.get_data(&*($wrapper))
            }
        }
    }
}

macro_rules! impl_verified_object {
    ($objname: ty) => {
        impl VerifiedObject for $objname {
            fn is_correct_type<T: Object>(object: &T) -> bool {
                object.class() == Class::from_existing(stringify!($objname))
            }

            fn error_message() -> &'static str {
                concat!("Not a ", stringify!($objname), " object")
            }
        }
    };
}

/// Helper macro to simplify type checking in the ruby-interfacing functions.
///
/// # Return
///
/// If called with only the object to check, this returns NIL after raising an exception.
/// If called with more arguments, the other things will be returned.
/// E.G.:
///
/// ```ignore
/// let obj1 = typecheck!(obj1); // returns `obj` or raises exception
///
/// // returns `obj` or raises exception and returns AnyObject (Boolean -> false):
/// let obj2 = typecheck!(obj2 or return any Boolean::new(false));
///
/// // returns `obj` or raises excpetion and returns Boolean -> false
/// let obj3 = typecheck!(obj3 or return Boolean::new(false));
/// ```
///
macro_rules! typecheck {
    ($obj: ident)                          => { typecheck!($obj or return NilClass::new()) };
    ($obj: ident or return any $els: expr) => { typecheck!($obj or return $els.to_any_object()) };
    ($obj: ident or return $els: expr)     => {
        match $obj {
            Ok(o)  => o,
            Err(e) => {
                VM::raise(e.to_exception(), e.description());
                return $els
            },
        }
    };

}

#[allow(unused_variables)]
pub mod storeid {
    use std::path::PathBuf;

    use ruru::{Class, Object, AnyObject, Boolean, RString, NilClass, VerifiedObject};

    use libimagstore::storeid::StoreId;
    use store::Unwrap;
    use store::Wrap;

    wrappable_struct!(StoreId, StoreIdWrapper, STOREID_WRAPPER);
    class!(RStoreId);
    impl_wrap!(StoreId, STOREID_WRAPPER);
    impl_unwrap!(RStoreId, StoreId, STOREID_WRAPPER);
    impl_verified_object!(RStoreId);


    methods!(
        RStoreId,
        itself,

        fn r_storeid_new(base: RString, id: RString) -> AnyObject {
            let base = match base.map(|b| b.to_string()).map(PathBuf::from) {
                Ok(base) => base,
                Err(e) => {
                    // TODO: Exception!
                    error!("Building StoreId object failed: {:?}", e);
                    return AnyObject::from(NilClass::new().value());
                },
            };

            let id = match id.map(|id| id.to_string()).map(PathBuf::from) {
                Ok(id) => id,
                Err(e) => {
                    // TODO: Exception!
                    error!("Building StoreId object failed: {:?}", e);
                    return AnyObject::from(NilClass::new().value());
                },
            };

            match StoreId::new(Some(base), id) {
                Ok(sid) => Class::from_existing("RStoreId").wrap_data(sid, &*STOREID_WRAPPER),
                Err(e) => {
                    // TODO: Exception!
                    error!("Building StoreId object failed: {:?}", e);
                    return AnyObject::from(NilClass::new().value());
                },
            }
        }

        fn r_storeid_new_baseless(id: RString) -> AnyObject {
            let id = match id.map(|id| id.to_string()).map(PathBuf::from) {
                Ok(id) => id,
                Err(e) => {
                    // TODO: Exception!
                    error!("Building StoreId object failed: {:?}", e);
                    return AnyObject::from(NilClass::new().value());
                },
            };

            match StoreId::new(None, id) {
                Ok(sid) => Class::from_existing("RStoreId").wrap_data(sid, &*STOREID_WRAPPER),
                Err(e) => {
                    // TODO: Exception!
                    error!("Building StoreId object failed: {:?}", e);
                    return AnyObject::from(NilClass::new().value());
                },
            }
        }

        fn r_storeid_without_base() -> RStoreId {
            let withoutbase : StoreId = itself.get_data(&*STOREID_WRAPPER).clone().without_base();
            Class::from_existing("RStoreId").wrap_data(withoutbase, &*STOREID_WRAPPER)
        }

        fn r_storeid_with_base(base: RString) -> AnyObject {
            let base : PathBuf = match base.map(|b| b.to_string()).map(PathBuf::from) {
                Ok(pb) => pb,
                Err(e) => {
                    // TODO: Exception!
                    error!("Error: {:?}", e);
                    return AnyObject::from(NilClass::new().value());
                },
            };
            let withoutbase : StoreId = itself.get_data(&*STOREID_WRAPPER).clone().with_base(base);
            Class::from_existing("RStoreId").wrap_data(withoutbase, &*STOREID_WRAPPER)
        }

        fn r_storeid_into_pathbuf() -> AnyObject {
            itself.get_data(&*STOREID_WRAPPER)
                .clone()
                .into_pathbuf()
                // TODO: No unwraps
                .map(|pb| pb.to_str().map(String::from).unwrap())
                .as_ref()
                .map(|s| AnyObject::from(RString::new(s).value()))
                // TODO: Exception!
                .unwrap_or(AnyObject::from(NilClass::new().value()))
        }

        fn r_storeid_exists() -> Boolean {
            Boolean::new(itself.get_data(&*STOREID_WRAPPER).exists())
        }

        fn r_storeid_to_str() -> AnyObject {
            itself.get_data(&*STOREID_WRAPPER)
                .to_str()
                .as_ref()
                .map(|s| AnyObject::from(RString::new(s).value()))
                // TODO: Exception!
                .unwrap_or(AnyObject::from(NilClass::new().value()))
        }

        fn r_storeid_local() -> RString {
            let local = itself.get_data(&*STOREID_WRAPPER).local();
            let local = local.to_str().unwrap(); // TODO: No unwraps
            RString::new(local)
        }

    );

    pub fn setup() -> Class {
        let mut class = Class::new("RStoreId", None);
        class.define(|itself| {
            itself.def_self("new"          , r_storeid_new);
            itself.def_self("new_baseless" , r_storeid_new_baseless);

            itself.def("without_base"      , r_storeid_without_base);
            itself.def("with_base"         , r_storeid_with_base);
            itself.def("into_pathbuf"      , r_storeid_into_pathbuf);
            itself.def("exists"            , r_storeid_exists);
            itself.def("to_str"            , r_storeid_to_str);
            itself.def("local"             , r_storeid_local);
        });
        class
    }

}

#[allow(unused_variables)]
pub mod store {
    pub mod entry {
        use std::collections::BTreeMap;
        use std::error::Error;
        use std::ops::Deref;
        use std::ops::DerefMut;

        use ruru::{Class, Object, AnyObject, Boolean, RString, VM, Hash, NilClass, VerifiedObject};

        use libimagstore::store::FileLockEntry as FLE;
        use libimagstore::store::EntryHeader;
        use libimagstore::store::EntryContent;
        use libimagstore::store::Entry;

        use ruby_utils::IntoToml;
        use toml_utils::IntoRuby;
        use store::Wrap;
        use store::Unwrap;

        pub struct FLECustomWrapper(Box<FLE<'static>>);

        impl Deref for FLECustomWrapper {
            type Target = Box<FLE<'static>>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for FLECustomWrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        wrappable_struct!(FLECustomWrapper, FileLockEntryWrapper, FLE_WRAPPER);
        class!(RFileLockEntry);
        impl_unwrap!(RFileLockEntry, FLECustomWrapper, FLE_WRAPPER);
        impl_verified_object!(RFileLockEntry);

        methods!(
            RFileLockEntry,
            itself,

            fn r_get_location() -> AnyObject {
                itself.get_data(&*FLE_WRAPPER).get_location().clone().wrap()
            }

            fn r_get_header() -> AnyObject {
                itself.get_data(&*FLE_WRAPPER).get_header().clone().wrap()
            }

            fn r_set_header(hdr: Hash) -> NilClass {
                use ruby_utils::IntoToml;
                use toml_utils::IntoRuby;
                use toml::Value;

                let mut header = itself.get_data(&*FLE_WRAPPER).get_header_mut();

                match typecheck!(hdr or return NilClass::new()).into_toml() {
                    Value::Table(t) => *header = EntryHeader::from(t),
                    _ => {
                        let ec = Class::from_existing("RuntimeError");
                        VM::raise(ec, "Something weird happened. Hash seems to be not a Hash");
                    },
                };
                NilClass::new()
            }

            fn r_get_content() -> AnyObject {
                itself.get_data(&*FLE_WRAPPER).get_content().clone().wrap()
            }

            fn r_set_content(ctt: RString) -> NilClass {
                use ruby_utils::IntoToml;
                use toml_utils::IntoRuby;
                use toml::Value;

                let mut content = itself.get_data(&*FLE_WRAPPER).get_content_mut();

                match typecheck!(ctt).into_toml() {
                    Value::String(s) => *content = s,
                    _ => {
                        let ec = Class::from_existing("RuntimeError");
                        VM::raise(ec, "Something weird happened. String seems to be not a String");
                    },
                }

                NilClass::new()
            }

        );

        wrappable_struct!(EntryHeader, EntryHeaderWrapper, ENTRY_HEADER_WRAPPER);
        class!(REntryHeader);
        impl_wrap!(EntryHeader, ENTRY_HEADER_WRAPPER);
        impl_unwrap!(REntryHeader, EntryHeader, ENTRY_HEADER_WRAPPER);
        impl_verified_object!(REntryHeader);

        methods!(
            REntryHeader,
            itself,

            fn r_entry_header_new() -> AnyObject {
                EntryHeader::new().wrap()
            }

            fn r_entry_header_insert(spec: RString, obj: AnyObject) -> Boolean {
                let spec = typecheck!(spec or return Boolean::new(false)).to_string();
                let obj = obj.unwrap(); // possibly not safe... TODO

                match itself.get_data(&*ENTRY_HEADER_WRAPPER).insert(&spec, obj.into_toml()) {
                    Ok(b) => Boolean::new(b),
                    Err(e) => {
                        VM::raise(Class::from_existing("RuntimeError"), e.description());
                        Boolean::new(false)
                    }
                }
            }

            fn r_entry_header_set(spec: RString, obj: AnyObject) -> AnyObject {
                use ruru::NilClass;

                let spec = typecheck!(spec or return any Boolean::new(false)).to_string();
                let obj = obj.unwrap(); // possibly not safe... TODO

                match itself.get_data(&*ENTRY_HEADER_WRAPPER).set(&spec, obj.into_toml()) {
                    Ok(Some(v)) => v.into_ruby(),
                    Ok(None)    => NilClass::new().to_any_object(),
                    Err(e) => {
                        VM::raise(Class::from_existing("RuntimeError"), e.description());
                        return Boolean::new(false).to_any_object();
                    }
                }
            }

            fn r_entry_header_get(spec: RString) -> AnyObject {
                use ruru::NilClass;

                let spec = typecheck!(spec or return any Boolean::new(false)).to_string();

                match itself.get_data(&*ENTRY_HEADER_WRAPPER).read(&spec) {
                    Ok(Some(v)) => v.into_ruby(),
                    Ok(None)    => NilClass::new().to_any_object(),
                    Err(e) => {
                        VM::raise(Class::from_existing("RuntimeError"), e.description());
                        return Boolean::new(false).to_any_object();
                    }
                }
            }

        );

        wrappable_struct!(EntryContent, EntryContentWrapper, ENTRY_CONTENT_WRAPPER);
        class!(REntryContent);
        impl_wrap!(EntryContent, ENTRY_CONTENT_WRAPPER);
        impl_unwrap!(REntryContent, EntryContent, ENTRY_CONTENT_WRAPPER);

        wrappable_struct!(Entry, EntryWrapper, ENTRY_WRAPPER);
        class!(REntry);
        impl_unwrap!(REntry, Entry, ENTRY_WRAPPER);

        pub fn setup_filelockentry() -> Class {
            let mut class = Class::new("RFileLockEntry", None);
            class.define(|itself| {
                itself.def("location", r_get_location);
                itself.def("header"  , r_get_header);
                itself.def("header=" , r_set_header);
                itself.def("content" , r_get_content);
                itself.def("content=", r_set_content);
            });
            class
        }

        pub fn setup_entryheader() -> Class {
            let mut class = Class::new("REntryHeader", None);
            class.define(|itself| {
                itself.def("insert", r_entry_header_insert);
                itself.def("set"   , r_entry_header_set);
                itself.def("read"  , r_entry_header_get);
            });
            class
        }

        pub fn setup_entrycontent() -> Class {
            let string = Class::from_existing("String");
            let mut class = Class::new("REntryContent", Some(&string));
            class
        }
    }

    use libimagstore::store::Store;
    use libimagerror::trace::trace_error;
    use std::error::Error;
    use std::ops::Deref;
    use std::ops::DerefMut;

    use ruru::{Class, Object, AnyObject, Boolean, RString, VM, Hash, NilClass, VerifiedObject};

    use ruby_utils::IntoToml;
    use toml_utils::IntoRuby;
    use store::Wrap;
    use store::Unwrap;

    wrappable_struct!(Store, StoreWrapper, STORE_WRAPPER);
    class!(RStore);
    impl_wrap!(Store, STORE_WRAPPER);
    impl_unwrap!(RStore, Store, STORE_WRAPPER);
    impl_verified_object!(RStore);

    use store::storeid::RStoreId;
    use store::store::entry::RFileLockEntry;

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
            unimplemented!()
        }

        // Save a FileLockEntry in a new path inside the store, move the RFileLockEntry
        //
        // # Returns:
        //
        // On success: Nil
        // On error: Nil + Exception
        //
        fn save_as(fle: RFileLockEntry, sid: RStoreId) -> NilClass {
            unimplemented!()
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

            if let Err(e) = itself.get_data(&*STORE_WRAPPER).move_by_id(old, nw) {
                trace_error(&e);
                VM::raise(Class::from_existing("RuntimeError"), e.description());
            }

            NilClass::new()
        }

        // Get the path of the store object
        //
        // # Returns:
        //
        // A RString
        //
        fn path() -> RString {
            itself.get_data(&*STORE_WRAPPER)
                .path()
                .clone()
                .to_str()
                .map(RString::new)
                .unwrap_or(RString::new(""))
        }

    );

}

