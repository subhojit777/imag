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
use std::error::Error;
use std::ops::Deref;
use std::ops::DerefMut;

use ruru::{Class, Object, AnyObject, Boolean, RString, VM, Hash, NilClass, VerifiedObject};

use libimagstore::store::FileLockEntry as FLE;
use libimagstore::store::EntryHeader;
use libimagstore::store::EntryContent;
use libimagstore::store::Entry;
use libimagstore::storeid::StoreId;

use ruby_utils::IntoToml;
use toml_utils::IntoRuby;
use util::Wrap;
use util::Unwrap;
use cache::FILE_LOCK_ENTRY_CACHE;

pub struct FileLockEntryHandle(StoreId);

impl FileLockEntryHandle {
    pub fn new(id: StoreId) -> FileLockEntryHandle {
        FileLockEntryHandle(id)
    }
}

impl Deref for FileLockEntryHandle {
    type Target = StoreId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FileLockEntryHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

wrappable_struct!(FileLockEntryHandle, FileLockEntryWrapper, FLE_WRAPPER);
class!(RFileLockEntry);
impl_unwrap!(RFileLockEntry, FileLockEntryHandle, FLE_WRAPPER);
impl_wrap!(FileLockEntryHandle, FLE_WRAPPER);
impl_verified_object!(RFileLockEntry);

/// Helper macro for operating on FILE_LOCK_ENTRY_CACHE object
///
/// This macro fetches an ARC from the FILE_LOCK_ENTRY_CACHE, then locks the Mutex inside it
/// and calls the $operation on the element inside the Mutex, this synchronizing the
/// operation on the FILE_LOCK_ENTRY_CACHE.
///
/// Yes, this is performance-wise not very elegant, but we're working with Ruby, and we need
/// to cache objects (why, see documentation for FILE_LOCK_ENTRY_CACHE).
///
#[macro_export]
macro_rules! operate_on_fle_cache {
    (mut |$name: ident| $operation: block) => {{
        use cache::FILE_LOCK_ENTRY_CACHE;

        let arc = FILE_LOCK_ENTRY_CACHE.clone();
        {
            let lock = arc.lock();
            match lock {
                Ok(mut $name) => { $operation },
                Err(e) => {
                    VM::raise(Class::from_existing("RuntimeError"), e.description());
                    NilClass::new().to_any_object()
                }
            }
        }
    }};
    (|$name: ident| $operation: block) => {{
        use cache::FILE_LOCK_ENTRY_CACHE;

        let arc = FILE_LOCK_ENTRY_CACHE.clone();
        {
            let lock = arc.lock();
            match lock {
                Ok($name) => { $operation },
                Err(e) => {
                    VM::raise(Class::from_existing("RuntimeError"), e.description());
                    NilClass::new().to_any_object()
                }
            }
        }
    }};
}


methods!(
    RFileLockEntry,
    itself,

    fn r_get_location() -> AnyObject {
        operate_on_fle_cache!(|hm| {
            match hm.get(itself.get_data(&*FLE_WRAPPER)) {
                Some(el) => el.get_location().clone().wrap(),
                None => {
                    VM::raise(Class::from_existing("RuntimeError"),
                            "Tried to operate on non-existing object");
                    NilClass::new().to_any_object()
                }
            }
        })
    }

    fn r_get_header() -> AnyObject {
        operate_on_fle_cache!(|hm| {
            match hm.get(itself.get_data(&*FLE_WRAPPER)) {
                Some(el) => el.get_header().clone().wrap(),
                None => {
                    VM::raise(Class::from_existing("RuntimeError"),
                            "Tried to operate on non-existing object");
                    NilClass::new().to_any_object()
                }
            }
        })
    }

    fn r_set_header(hdr: Hash) -> NilClass {
        use ruby_utils::IntoToml;
        use toml_utils::IntoRuby;
        use toml::Value;

        let entryheader = match typecheck!(hdr or return NilClass::new()).into_toml() {
            Value::Table(t) => EntryHeader::from(t),
            _ => {
                let ec = Class::from_existing("RuntimeError");
                VM::raise(ec, "Something weird happened. Hash seems to be not a Hash");
                return NilClass::new();
            },
        };

        operate_on_fle_cache!(mut |hm| {
            match hm.get_mut(itself.get_data(&*FLE_WRAPPER)) {
                Some(mut el) => {
                    *el.get_header_mut() = entryheader;
                },
                None => {
                    VM::raise(Class::from_existing("RuntimeError"),
                            "Tried to operate on non-existing object");
                }
            }
            NilClass::new().to_any_object()
        });

        NilClass::new()
    }

    fn r_get_content() -> AnyObject {
        operate_on_fle_cache!(|hm| {
            match hm.get(itself.get_data(&*FLE_WRAPPER)) {
                Some(el) => el.get_content().clone().wrap(),
                None => NilClass::new().to_any_object()
            }
        })
    }

    fn r_set_content(ctt: RString) -> NilClass {
        use ruby_utils::IntoToml;
        use toml_utils::IntoRuby;
        use toml::Value;

        let content = match typecheck!(ctt).into_toml() {
            Value::String(s) => s,
            _ => {
                let ec = Class::from_existing("RuntimeError");
                VM::raise(ec, "Something weird happened. String seems to be not a String");
                return NilClass::new();
            },
        };

        operate_on_fle_cache!(mut |hm| {
            match hm.get_mut(itself.get_data(&*FLE_WRAPPER)) {
                Some(el) => {
                    *el.get_content_mut() = content;
                },
                None => {
                    VM::raise(Class::from_existing("RuntimeError"),
                            "Tried to operate on non-existing object");
                }
            }
            NilClass::new().to_any_object()
        });

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
