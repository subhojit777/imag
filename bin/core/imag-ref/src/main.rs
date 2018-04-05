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

#[macro_use] extern crate log;
extern crate clap;

extern crate libimagstore;
#[macro_use] extern crate libimagrt;
extern crate libimagentryref;
extern crate libimagerror;
extern crate libimaginteraction;
extern crate libimagutil;

mod ui;
use ui::build_ui;

use std::path::PathBuf;
use std::process::exit;

use libimagerror::trace::MapErrTrace;
use libimagrt::setup::generate_runtime_setup;
use libimagrt::runtime::Runtime;
use libimagstore::storeid::IntoStoreId;
use libimagentryref::reference::Ref;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-ref",
                                    &version,
                                    "Reference files outside of the store",
                                    build_ui);
    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call: {}", name);
            match name {
                "deref"  => deref(&rt),
                "remove" => remove(&rt),
                other => {
                    debug!("Unknown command");
                    let _ = rt.handle_unknown_subcommand("imag-ref", other, rt.cli())
                        .map_err_trace_exit_unwrap(1)
                        .code()
                        .map(::std::process::exit);
                },
            };
        });
}

fn deref(rt: &Runtime) {
    let cmd  = rt.cli().subcommand_matches("deref").unwrap();
    let id   = cmd.value_of("ID")
        .map(String::from)
        .map(PathBuf::from)
        .unwrap() // saved by clap
        .into_storeid()
        .map_err_trace_exit_unwrap(1);

    match rt.store().get(id.clone()).map_err_trace_exit_unwrap(1) {
        Some(entry) => entry
            .get_path()
            .map_err_trace_exit_unwrap(1)
            .to_str()
            .ok_or_else(|| {
                error!("Could not transform path into string!");
                exit(1)
            })
            .map(|s| info!("{}", s))
            .ok(), // safe here because we exited already in the error case
        None => {
            error!("No entry for id '{}' found", id);
            exit(1)
        },
    };
}

fn remove(rt: &Runtime) {
    use libimaginteraction::ask::ask_bool;

    let cmd  = rt.cli().subcommand_matches("remove").unwrap();
    let yes  = cmd.is_present("yes");
    let id   = cmd.value_of("ID")
        .map(String::from)
        .map(PathBuf::from)
        .unwrap() // saved by clap
        .into_storeid()
        .map_err_trace_exit_unwrap(1);

    match rt.store().get(id.clone()).map_err_trace_exit_unwrap(1) {
        Some(mut entry) => {
            if yes || ask_bool(&format!("Delete ref from entry '{}'", id), None) {
                let _ = entry.remove_ref().map_err_trace_exit_unwrap(1);
            } else {
                info!("Aborted");
            }
        },
        None => {
            error!("No entry for id '{}' found", id);
            exit(1)
        },
    };
}

