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
#[macro_use] extern crate version;
extern crate toml;
extern crate toml_query;
extern crate kairos;

extern crate libimaghabit;
extern crate libimagstore;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;

use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::{MapErrTrace, trace_error};
use libimaghabit::store::HabitStore;
use libimaghabit::habit::builder::HabitBuilder;
use libimaghabit::habit::HabitTemplate;

mod ui;

fn main() {
    let rt = generate_runtime_setup("imag-habit",
                                    &version!()[..],
                                    "Habit tracking tool",
                                    ui::build_ui);


    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "create" => create(&rt),
                "delete" => delete(&rt),
                "list"   => list(&rt),
                "show"   => show(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                    exit(1)
                },
            }
        });
}

fn create(rt: &Runtime) {
    unimplemented!()
}

fn delete(rt: &Runtime) {
    unimplemented!()
}

fn list(rt: &Runtime) {
    unimplemented!()
}

fn show(rt: &Runtime) {
    unimplemented!()
}

