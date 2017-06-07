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

#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_mut,
    unused_qualifications,
    while_true,
)]

#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate fs2;
extern crate glob;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate toml;
#[cfg(test)] extern crate tempdir;
extern crate semver;
extern crate crossbeam;
extern crate walkdir;
extern crate itertools;
#[macro_use] extern crate is_match;

#[macro_use] extern crate libimagerror;
extern crate libimagutil;

#[macro_use] mod util;

pub mod storeid;
pub mod error;
pub mod store;
mod configuration;
mod file_abstraction;
pub mod toml_ext;

