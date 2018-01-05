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

#[macro_use] extern crate error_chain;
#[macro_use] extern crate is_match;
extern crate filters;
extern crate toml_query;
extern crate chrono;
extern crate toml;

#[macro_use] extern crate libimagstore;
extern crate libimagerror;
extern crate libimagentrylink;
#[macro_use] extern crate libimagentryutil;
extern crate libimagutil;

module_entry_path_mod!("flashcard");

pub mod card;
pub mod error;
pub mod group;
pub mod iter;
pub mod pathes;
pub mod session;
pub mod store;

