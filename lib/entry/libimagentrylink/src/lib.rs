//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

#![recursion_limit="256"]

#![deny(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

extern crate itertools;
#[macro_use] extern crate log;
extern crate toml;
extern crate toml_query;
extern crate url;
extern crate sha1;
extern crate hex;
#[macro_use] extern crate is_match;
#[macro_use] extern crate error_chain;

#[cfg(test)]
extern crate env_logger;

#[macro_use] extern crate libimagstore;
extern crate libimagerror;
extern crate libimagutil;

module_entry_path_mod!("links");

pub mod error;
pub mod external;
pub mod internal;

