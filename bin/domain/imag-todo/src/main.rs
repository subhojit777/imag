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
extern crate toml_query;
#[macro_use] extern crate is_match;

#[macro_use] extern crate libimagrt;
extern crate libimagerror;
extern crate libimagtodo;

use std::process::{Command, Stdio};
use std::io::stdin;
use std::io::Write;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagtodo::taskstore::TaskStore;
use libimagerror::trace::{MapErrTrace, trace_error};
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;

mod ui;

use ui::build_ui;
fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-todo",
                                    &version,
                                    "Interface with taskwarrior",
                                    build_ui);

    match rt.cli().subcommand_name() {
        Some("tw-hook") => tw_hook(&rt),
        Some("list") => list(&rt),
        Some(other) => {
            debug!("Unknown command");
            let _ = rt.handle_unknown_subcommand("imag-todo", other, rt.cli())
                .map_err_trace_exit_unwrap(1)
                .code()
                .map(::std::process::exit);
        }
        None => {
            warn!("No command");
        },
    } // end match scmd
} // end main

fn tw_hook(rt: &Runtime) {
    let subcmd = rt.cli().subcommand_matches("tw-hook").unwrap();
    if subcmd.is_present("add") {
        let stdin = stdin();

        // implements BufRead which is required for `Store::import_task_from_reader()`
        let stdin = stdin.lock();

        let (_, line, uuid ) = rt
            .store()
            .import_task_from_reader(stdin)
            .map_err_trace_exit_unwrap(1);

        let _ = writeln!(rt.stdout(), "{}\nTask {} stored in imag", line, uuid)
            .to_exit_code()
            .unwrap_or_exit();

    } else if subcmd.is_present("delete") {
        // The used hook is "on-modify". This hook gives two json-objects
        // per usage und wants one (the second one) back.
        let stdin         = stdin();
        rt.store().delete_tasks_by_imports(stdin.lock()).map_err_trace().ok();
    } else {
        // Should not be possible, as one argument is required via
        // ArgGroup
        unreachable!();
    }
}

fn list(rt: &Runtime) {
    use toml_query::read::TomlValueReadTypeExt;

    let subcmd  = rt.cli().subcommand_matches("list").unwrap();
    let verbose = subcmd.is_present("verbose");

    // Helper for toml_query::read::TomlValueReadExt::read() return value, which does only
    // return Result<T> instead of Result<Option<T>>, which is a real inconvenience.
    //
    let no_identifier = |e: &::toml_query::error::Error| -> bool {
        is_match!(e.kind(), &::toml_query::error::ErrorKind::IdentifierNotFoundInDocument(_))
    };

    let res = rt.store().all_tasks() // get all tasks
        .map(|iter| { // and if this succeeded
            // filter out the ones were we can read the uuid
            let uuids : Vec<_> = iter.filter_map(|storeid| {
                match rt.store().retrieve(storeid) {
                    Ok(fle) => {
                        match fle.get_header().read_string("todo.uuid") {
                            Ok(Some(ref u)) => Some(u.clone()),
                            Ok(None) => {
                                error!("Header missing field in {}", fle.get_location());
                                None
                            },
                            Err(e) => {
                                if !no_identifier(&e) {
                                    trace_error(&e);
                                }
                                None
                            }
                        }
                    },
                    Err(e) => {
                        trace_error(&e);
                        None
                    },
                }
            })
            .collect();

            // compose a `task` call with them, ...
            let outstring = if verbose { // ... if verbose
                let output = Command::new("task")
                    .stdin(Stdio::null())
                    .args(&uuids)
                    .spawn()
                    .unwrap_or_else(|e| {
                        error!("Failed to execute `task` on the commandline: {:?}. I'm dying now.", e);
                        ::std::process::exit(1)
                    })
                    .wait_with_output()
                    .unwrap_or_else(|e| panic!("failed to unwrap output: {}", e));

                String::from_utf8(output.stdout)
                    .unwrap_or_else(|e| panic!("failed to execute: {}", e))
            } else { // ... else just join them
                uuids.join("\n")
            };

            // and then print that
            let _ = writeln!(rt.stdout(), "{}", outstring).to_exit_code().unwrap_or_exit();
        });

    res.map_err_trace().ok();
}

