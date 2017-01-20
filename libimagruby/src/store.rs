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
pub mod storeid {
    use std::path::PathBuf;

    use ruru::{Class, Object, AnyObject, Boolean, RString, NilClass};

    use libimagstore::storeid::StoreId;

    wrappable_struct!(StoreId, StoreIdWrapper, STOREID_WRAPPER);
    class!(RStoreId);

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

        use ruru::{Class, Object, Array, Hash, Fixnum, Float, Symbol, AnyObject, Boolean, RString, VM};

        use libimagstore::store::FileLockEntry as FLE;
        use libimagstore::store::EntryHeader;
        use libimagstore::store::EntryContent;
        use libimagstore::store::Entry;

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

        wrappable_struct!(EntryHeader, EntryHeaderWrapper, ENTRY_HEADER_WRAPPER);
        class!(REntryHeader);
        methods!(
            REntryHeader,
            itself,

            fn r_entry_header_new() -> AnyObject {
                Class::from_existing("REntryHeader")
                    .wrap_data(EntryHeader::new(), &*ENTRY_HEADER_WRAPPER)
            }

            fn r_entry_header_insert(spec: RString, obj: AnyObject) -> Boolean {
                use toml::Value;
                use ruru::types::ValueType;

                fn do_insert(eh: &mut EntryHeader, spec: &str, v: Value) -> Boolean {
                    match eh.insert(spec, v) {
                        Ok(b) => Boolean::new(b),
                        Err(e) => {
                            VM::raise(Class::from_existing("RuntimeError"), e.description());
                            return Boolean::new(false);
                        }
                    }
                }

                fn to_value(obj: AnyObject) -> Result<Value, ()> {
                    match obj.value().ty() {
                        ValueType::Fixnum =>
                            Ok(Value::Integer(obj.try_convert_to::<Fixnum>().unwrap().to_i64())),

                        ValueType::Float =>
                            Ok(Value::Float(obj.try_convert_to::<Float>().unwrap().to_f64())),

                        ValueType::RString =>
                            Ok(Value::String(obj.try_convert_to::<RString>().unwrap().to_string())),

                        ValueType::Symbol =>
                            Ok(Value::String(obj.try_convert_to::<Symbol>().unwrap().to_string())),

                        _ => Err(()),
                    }
                }

                if let Err(ref error) = spec { // raise exception if "spec" is not a String
                    VM::raise(error.to_exception(), error.description());
                    return Boolean::new(false);
                }

                let spec = spec.unwrap().to_string(); // safe because of check above.
                let obj = obj.unwrap(); // possibly not safe... TODO
                match obj.value().ty() {
                    ValueType::Fixnum => {
                        let i = obj.try_convert_to::<Fixnum>().unwrap().to_i64();
                        do_insert(itself.get_data(&*ENTRY_HEADER_WRAPPER), &spec, Value::Integer(i))
                    }

                    ValueType::Float => {
                        let f = obj.try_convert_to::<Float>().unwrap().to_f64();
                        do_insert(itself.get_data(&*ENTRY_HEADER_WRAPPER), &spec, Value::Float(f))
                    }

                    ValueType::RString => {
                        let s = obj.try_convert_to::<RString>().unwrap().to_string();
                        do_insert(itself.get_data(&*ENTRY_HEADER_WRAPPER), &spec, Value::String(s))
                    }

                    ValueType::Symbol => {
                        let s = obj.try_convert_to::<Symbol>().unwrap().to_string();
                        do_insert(itself.get_data(&*ENTRY_HEADER_WRAPPER), &spec, Value::String(s))
                    }

                    ValueType::Array => {
                        let vals = obj.try_convert_to::<Array>()
                            .unwrap()
                            .into_iter()
                            .map(to_value)
                            .map(|el| {
                                let e_class = Class::from_existing("ArgumentError");
                                let err = "Arrays can only hold 'Fixnum', 'Float', 'String' or 'Symbol' in this API";
                                el.map_err(|_| VM::raise(e_class, err))
                            })
                            .filter_map(|e| e.ok())
                            .collect::<Vec<Value>>();

                        do_insert(itself.get_data(&*ENTRY_HEADER_WRAPPER), &spec, Value::Array(vals))
                    }

                    ValueType::Hash => {
                        let mut btm = BTreeMap::new();
                        obj.try_convert_to::<Hash>()
                            .unwrap()
                            .each(|key, value| {
                                let key = match key.value().ty() {
                                    ValueType::RString => obj.try_convert_to::<RString>().unwrap().to_string(),
                                    ValueType::Symbol  => obj.try_convert_to::<Symbol>().unwrap().to_string(),
                                    _ => {
                                        let e_class = Class::from_existing("ArgumentError");
                                        let err = "Hash must have 'String' or 'Symbol' as Key";
                                        VM::raise(e_class, err);
                                        return; // from closure
                                    }
                                };
                                let value = match to_value(value) {
                                    Err(e) => {
                                        let e_class = Class::from_existing("ArgumentError");
                                        let err = "Hash must have 'Fixnum', 'Float', 'String' or 'Symbol' as value in this API";
                                        VM::raise(e_class, err);
                                        return; // from closure
                                    }
                                    Ok(v) => v,
                                };

                                btm.insert(key, value);
                            });

                        do_insert(itself.get_data(&*ENTRY_HEADER_WRAPPER), &spec, Value::Table(btm))
                    }

                    ValueType::Nil => {
                        VM::raise(Class::from_existing("ArgumentError"), "Unexpected Argument 'nil'");
                        return Boolean::new(false);
                    }

                    _ => {
                        VM::raise(Class::from_existing("ArgumentError"), "Unexpected Argument Type");
                        return Boolean::new(false);
                    }
                }

            }

        );

        wrappable_struct!(EntryContent, EntryContentWrapper, ENTRY_CONTENT_WRAPPER);
        class!(REntryContent);

        wrappable_struct!(Entry, EntryWrapper, ENTRY_WRAPPER);
        class!(REntry);

        pub fn setup_filelockentry() -> Class {
            let mut class = Class::new("RFileLockEntry", None);
            class
        }

        pub fn setup_entryheader() -> Class {
            let mut class = Class::new("REntryHeader", None);
            class
        }

        pub fn setup_entrycontent() -> Class {
            let string = Class::from_existing("String");
            let mut class = Class::new("REntryContent", Some(&string));
            class
        }
    }

}

