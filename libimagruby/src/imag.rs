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


use std::error::Error;

use ruru::{Class, RString, NilClass, VM, Object};

class!(Imag);

methods!(
    Imag,
    itself,

    fn r_log_trace(l: RString) -> NilClass {
        match l {
            Err(ref e) => VM::raise(e.to_exception(), e.description()),
            Ok(s)      => trace!("{}", s.to_string()),
        }
        NilClass::new()
    }

    fn r_log_debug(l: RString) -> NilClass {
        match l {
            Err(ref e) => VM::raise(e.to_exception(), e.description()),
            Ok(s)      => debug!("{}", s.to_string()),
        }
        NilClass::new()
    }

    fn r_log_info(l: RString) -> NilClass {
        match l {
            Err(ref e) => VM::raise(e.to_exception(), e.description()),
            Ok(s)      => info!("{}", s.to_string()),
        }
        NilClass::new()
    }

    fn r_log_warn(l: RString) -> NilClass {
        match l {
            Err(ref e) => VM::raise(e.to_exception(), e.description()),
            Ok(s)      => warn!("{}", s.to_string()),
        }
        NilClass::new()
    }

    fn r_log_error(l: RString) -> NilClass {
        match l {
            Err(ref e) => VM::raise(e.to_exception(), e.description()),
            Ok(s)      => error!("{}", s.to_string()),
        }
        NilClass::new()
    }

);

pub fn setup() -> Class {
    let mut class = Class::new("Imag", None);
    class.define(|itself| {
        itself.def_self("trace",    r_log_trace);
        itself.def_self("dbg",      r_log_debug);
        itself.def_self("debug",    r_log_debug);
        itself.def_self("info",     r_log_info);
        itself.def_self("warn",     r_log_warn);
        itself.def_self("error",    r_log_error);
    });
    class
}

