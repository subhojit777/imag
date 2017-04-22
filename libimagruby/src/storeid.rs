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

use std::path::PathBuf;

use ruru::{Class, Object, AnyObject, Boolean, RString, NilClass, VerifiedObject, VM};

use libimagstore::storeid::StoreId;
use util::Unwrap;
use util::Wrap;

wrappable_struct!(StoreId, StoreIdWrapper, STOREID_WRAPPER);
class!(RStoreId);
impl_wrap!(StoreId => STOREID_WRAPPER);
impl_unwrap!(RStoreId => StoreId => STOREID_WRAPPER);
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
        use std::error::Error;

        match itself.get_data(&*STOREID_WRAPPER).exists() {
            Ok(bool) => Boolean::new(bool),
            Err(e) => {
                VM::raise(Class::from_existing("RuntimeError"), e.description());
                Boolean::new(false)
            }
        }
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

