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

extern crate chrono;
#[macro_use] extern crate log;
extern crate toml;
extern crate toml_query;
extern crate itertools;
#[macro_use] extern crate error_chain;
extern crate filters;

#[macro_use] extern crate libimagstore;
#[macro_use] extern crate libimagentryutil;
extern crate libimagerror;
extern crate libimagentryedit;
extern crate libimagentryview;
extern crate libimagrt;

module_entry_path_mod!("diary");

pub mod config;
pub mod error;
pub mod diaryid;
pub mod diary;
pub mod is_in_diary;
pub mod entry;
pub mod iter;
pub mod viewer;

