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

use std::error::Error;

use ruru::{Class, Boolean, RString, NilClass, VM, Object};

use libimagrt::logger::ImagLogger;

class!(RImag);

methods!(
    RImag,
    itself,

    fn r_initialize_logger(debug: Boolean, verbose: Boolean, colored: Boolean) -> NilClass {
        use std::env::var as env_var;
        use env_logger;
        use log;
        use log::LogLevelFilter;

        let debug = match debug {
            Ok(d)      => d.to_bool(),
            Err(ref e) => {
                VM::raise(e.to_exception(), e.description());
                return NilClass::new();
            },
        };

        let verbose = match verbose {
            Ok(v)      => v.to_bool(),
            Err(ref e) => {
                VM::raise(e.to_exception(), e.description());
                return NilClass::new();
            },
        };

        let colored = match colored {
            Ok(c)      => c.to_bool(),
            Err(ref e) => {
                VM::raise(e.to_exception(), e.description());
                return NilClass::new();
            },
        };

        if env_var("IMAG_LOG_ENV").is_ok() {
            env_logger::init().unwrap();
        } else {
            let lvl = if debug {
                LogLevelFilter::Debug
            } else if verbose {
                LogLevelFilter::Info
            } else {
                LogLevelFilter::Warn
            };

            log::set_logger(|max_log_lvl| {
                max_log_lvl.set(lvl);
                debug!("Init logger with {}", lvl);
                let lgr = ImagLogger::new(lvl.to_log_level().unwrap())
                    .with_color(colored)
                    .with_prefix("[imag][ruby]".to_owned())
                    .with_dbg_file_and_line(false);
                Box::new(lgr)
            })
            .map_err(|_| {
                panic!("Could not setup logger");
            })
            .ok();
        }

        NilClass::new()
    }

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
    let mut class = Class::new("RImag", None);
    class.define(|itself| {
        itself.def_self("init_logger",  r_initialize_logger);
        itself.def_self("trace",        r_log_trace);
        itself.def_self("dbg",          r_log_debug);
        itself.def_self("debug",        r_log_debug);
        itself.def_self("info",         r_log_info);
        itself.def_self("warn",         r_log_warn);
        itself.def_self("error",        r_log_error);
    });
    class
}

