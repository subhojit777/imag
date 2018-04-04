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
#[macro_use]
extern crate log;

extern crate libimagentryannotation;
extern crate libimagentryedit;
extern crate libimagerror;
#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;

use std::io::Write;
use std::path::PathBuf;

use libimagentryannotation::annotateable::*;
use libimagentryannotation::annotation_fetcher::*;
use libimagentryannotation::error::AnnotationError as AE;
use libimagentryedit::edit::*;
use libimagerror::trace::MapErrTrace;
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::IntoStoreId;

mod ui;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-annotation",
                                    &version,
                                    "Add annotations to entries",
                                    ui::build_ui);

    rt.cli()
        .subcommand_name()
        .map(|name| {
            match name {
                "add"    => add(&rt),
                "remove" => remove(&rt),
                "list"   => list(&rt),
                other    => {
                    debug!("Unknown command");
                    let _ = rt.handle_unknown_subcommand("imag-annotation", other, rt.cli())
                        .map_err_trace_exit_unwrap(1)
                        .code()
                        .map(std::process::exit);
                },
            }
        });
}

fn add(rt: &Runtime) {
    let scmd            = rt.cli().subcommand_matches("add").unwrap(); // safed by main()
    let annotation_name = scmd.value_of("annotation_name").unwrap(); // safed by clap
    let entry_name      = scmd
        .value_of("entry")
        .map(PathBuf::from)
        .map(|pb| pb.into_storeid().map_err_trace_exit_unwrap(1))
        .unwrap(); // safed by clap

    let _ = rt.store()
        .get(entry_name)
        .map_err_trace_exit_unwrap(1)
        .ok_or(AE::from("Entry does not exist".to_owned()))
        .map_err_trace_exit_unwrap(1)
        .annotate(rt.store(), annotation_name)
        .map_err_trace_exit_unwrap(1)
        .edit_content(&rt)
        .map_err_trace_exit_unwrap(1);
}

fn remove(rt: &Runtime) {
    let scmd            = rt.cli().subcommand_matches("remove").unwrap(); // safed by main()
    let entry_name      = scmd.value_of("entry").unwrap(); // safed by clap
    let annotation_name = scmd.value_of("annotation_name").unwrap(); // safed by clap
    let delete          = scmd.is_present("delete-annotation");

    let mut entry = rt.store()
        .get(PathBuf::from(entry_name).into_storeid().map_err_trace_exit_unwrap(1))
        .map_err_trace_exit_unwrap(1)
        .ok_or(AE::from("Entry does not exist".to_owned()))
        .map_err_trace_exit_unwrap(1);

    let annotation = entry
        .denotate(rt.store(), annotation_name)
        .map_err_trace_exit_unwrap(1);

    if delete {
        debug!("Deleting annotation object");
        if let Some(an) = annotation {
            let loc = an.get_location().clone();
            drop(an);

            let _ = rt
                .store()
                .delete(loc)
                .map_err_trace_exit_unwrap(1);
        } else {
            warn!("Not having annotation object, cannot delete!");
        }
    } else {
        debug!("Not deleting annotation object");
    }
}

fn list(rt: &Runtime) {
    let scmd        = rt.cli().subcommand_matches("list").unwrap(); // safed by clap
    let with_text   = scmd.is_present("list-with-text");
    match scmd.value_of("entry").map(PathBuf::from) {
        Some(pb) => {
            let _ = rt
                .store()
                .get(pb.into_storeid().map_err_trace_exit_unwrap(1))
                .map_err_trace_exit_unwrap(1)
                .ok_or(AE::from("Entry does not exist".to_owned()))
                .map_err_trace_exit_unwrap(1)
                .annotations(rt.store())
                .map_err_trace_exit_unwrap(1)
                .enumerate()
                .map(|(i, a)| {
                    list_annotation(&rt, i, a.map_err_trace_exit_unwrap(1), with_text)
                })
                .collect::<Vec<_>>();
        }

        None => {
            // show them all
            let _ = rt
                .store()
                .all_annotations()
                .map_err_trace_exit_unwrap(1)
                .enumerate()
                .map(|(i, a)| {
                    list_annotation(&rt, i, a.map_err_trace_exit_unwrap(1), with_text)
                })
                .collect::<Vec<_>>();
        }
    }
}

fn list_annotation<'a>(rt: &Runtime, i: usize, a: FileLockEntry<'a>, with_text: bool) {
    let _ = if with_text {
        writeln!(rt.stdout(),
                 "--- {i: >5} | {id}\n{text}\n\n",
                 i = i,
                 id = a.get_location(),
                 text = a.get_content())
    } else {
        writeln!(rt.stdout(), "{: >5} | {}", i, a.get_location())
    }
    .to_exit_code()
    .unwrap_or_exit();
}

