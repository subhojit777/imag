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
extern crate toml;
#[cfg(test)] extern crate toml_query;
#[macro_use] extern crate error_chain;

#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimagerror;

#[cfg(test)]
#[macro_use]
extern crate libimagutil;

#[cfg(not(test))]
extern crate libimagutil;

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;

mod create;
mod delete;
mod dump;
mod error;
mod get;
mod retrieve;
mod ui;
mod update;
mod verify;
mod util;

use std::ops::Deref;

use create::create;
use delete::delete;
use dump::dump;
use get::get;
use retrieve::retrieve;
use ui::build_ui;
use update::update;
use verify::verify;

fn main() {
    let version = make_imag_version!();
    let mut rt = generate_runtime_setup("imag-store",
                                        &version,
                                        "Direct interface to the store. Use with great care!",
                                        build_ui);

    let command = rt.cli().subcommand_name().map(String::from);

    if let Some(command) = command {
        debug!("Call: {}", command);
        match command.deref() {
            "create"   => create(&rt),
            "delete"   => delete(&rt),
            "get"      => get(&rt),
            "retrieve" => retrieve(&rt),
            "update"   => update(&rt),
            "verify"   => verify(&rt),
            "dump"     => dump(&mut rt),
            other      => {
                debug!("Unknown command");
                let _ = rt.handle_unknown_subcommand("imag-store", other, rt.cli())
                    .map_err_trace_exit_unwrap(1)
                    .code()
                    .map(::std::process::exit);
            },
        };
    } else {
        debug!("No command");
    }
}

