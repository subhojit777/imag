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
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;
#[macro_use] extern crate libimagerror;

use libimagrt::setup::generate_runtime_setup;

mod create;
mod delete;
mod error;
mod get;
mod retrieve;
mod ui;
mod update;
mod verify;
mod util;

use create::create;
use delete::delete;
use get::get;
use retrieve::retrieve;
use ui::build_ui;
use update::update;
use verify::verify;

fn main() {
    let rt = generate_runtime_setup("imag-store",
                                    &version!()[..],
                                    "Direct interface to the store. Use with great care!",
                                    build_ui);

    rt.cli()
        .subcommand_name()
        .map_or_else(
            || {
                debug!("No command");
                // More error handling
            },
            |name| {
                debug!("Call: {}", name);
                match name {
                    "create"   => create(&rt),
                    "delete"   => delete(&rt),
                    "get"      => get(&rt),
                    "retrieve" => retrieve(&rt),
                    "update"   => update(&rt),
                    "verify"   => verify(&rt),
                    _ => {
                        debug!("Unknown command");
                        // More error handling
                    },
                };
            }
        )
}

