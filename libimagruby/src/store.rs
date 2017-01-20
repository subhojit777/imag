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

#[allow(unused_variables)]
pub mod storeid {
    use std::path::PathBuf;

    use ruru::{Class, Object, AnyObject, Boolean, RString, NilClass};

    use libimagstore::storeid::StoreId;

    wrappable_struct!(StoreId, StoreIdWrapper, STOREID_WRAPPER);
    class!(RStoreId);

    use store::Wrap;
    impl Wrap for StoreId {
        fn wrap(self) -> AnyObject {
            Class::from_existing("RStoreId").wrap_data(self, &*STOREID_WRAPPER)
        }
    }

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

        use ruru::{Class, Object, AnyObject, Boolean, RString, VM, Hash, NilClass};

        use libimagstore::store::FileLockEntry as FLE;
        use libimagstore::store::EntryHeader;
        use libimagstore::store::EntryContent;
        use libimagstore::store::Entry;

        use ruby_utils::IntoToml;
        use toml_utils::IntoRuby;
        use store::Wrap;

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

                if let Err(ref error) = hdr { // raise exception if "hdr" is not a Hash
                    VM::raise(error.to_exception(), error.description());
                    return NilClass::new();
                }

                let hdr = match hdr.unwrap().into_toml() {
                    Value::Table(t) => *header = EntryHeader::from(t),
                    _ => {
                        let ec = Class::from_existing("RuntimeError");
                        VM::raise(ec, "Something weird happened. Hash seems to be not a Hash");
                        return NilClass::new();
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

                if let Err(ref error) = ctt { // raise exception if "ctt" is not a String
                    VM::raise(error.to_exception(), error.description());
                    return NilClass::new();
                }

                let hdr = match ctt.unwrap().into_toml() {
                    Value::String(s) => *content = s,
                    _ => {
                        let ec = Class::from_existing("RuntimeError");
                        VM::raise(ec, "Something weird happened. String seems to be not a String");
                        return NilClass::new();
                    },
                };

                NilClass::new()
            }

        );

        wrappable_struct!(EntryHeader, EntryHeaderWrapper, ENTRY_HEADER_WRAPPER);
        class!(REntryHeader);

        impl Wrap for EntryHeader {
            fn wrap(self) -> AnyObject {
                Class::from_existing("REntryHeader").wrap_data(self, &*ENTRY_HEADER_WRAPPER)
            }
        }

        methods!(
            REntryHeader,
            itself,

            fn r_entry_header_new() -> AnyObject {
                EntryHeader::new().wrap()
            }

            fn r_entry_header_insert(spec: RString, obj: AnyObject) -> Boolean {
                if let Err(ref error) = spec { // raise exception if "spec" is not a String
                    VM::raise(error.to_exception(), error.description());
                    return Boolean::new(false);
                }

                let spec = spec.unwrap().to_string(); // safe because of check above.
                let obj = obj.unwrap(); // possibly not safe... TODO

                match itself.get_data(&*ENTRY_HEADER_WRAPPER).insert(&spec, obj.into_toml()) {
                    Ok(b) => Boolean::new(b),
                    Err(e) => {
                        VM::raise(Class::from_existing("RuntimeError"), e.description());
                        return Boolean::new(false);
                    }
                }
            }

            fn r_entry_header_set(spec: RString, obj: AnyObject) -> AnyObject {
                use ruru::NilClass;

                if let Err(ref error) = spec { // raise exception if "spec" is not a String
                    VM::raise(error.to_exception(), error.description());
                    return Boolean::new(false).to_any_object();
                }

                let spec = spec.unwrap().to_string(); // safe because of check above.
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

                if let Err(ref error) = spec { // raise exception if "spec" is not a String
                    VM::raise(error.to_exception(), error.description());
                    return Boolean::new(false).to_any_object();
                }

                let spec = spec.unwrap().to_string(); // safe because of check above.

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

        impl Wrap for EntryContent {
            fn wrap(self) -> AnyObject {
                Class::from_existing("REntryContent").wrap_data(self, &*ENTRY_CONTENT_WRAPPER)
            }
        }

        wrappable_struct!(Entry, EntryWrapper, ENTRY_WRAPPER);
        class!(REntry);

        pub fn setup_filelockentry() -> Class {
            let mut class = Class::new("RFileLockEntry", None);
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

}

