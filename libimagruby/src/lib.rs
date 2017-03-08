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

#[macro_use] extern crate ruru;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate toml;
extern crate uuid;

#[macro_use] extern crate libimagerror;
extern crate libimagstore;
extern crate libimagstorestdhook;
extern crate libimagrt;
#[macro_use] extern crate libimagutil;

#[macro_use] mod util;
#[macro_use] pub mod store;
mod cache;

pub mod error;
pub mod entry;
pub mod imag;
pub mod ruby_utils;
pub mod storeid;
pub mod toml_utils;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Init_liblibimagruby() {
    self::error::setup();
    self::store::setup();
    self::storeid::setup();
    self::entry::setup_filelockentry();
    self::entry::setup_entryheader();
    self::entry::setup_entrycontent();
    self::imag::setup();
}

