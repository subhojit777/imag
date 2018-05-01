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

extern crate libimagentryedit;
extern crate libimagerror;
#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;

use std::path::PathBuf;
use std::io::Read;

use libimagerror::trace::MapErrTrace;
use libimagerror::iter::TraceIterator;
use libimagentryedit::edit::Edit;
use libimagentryedit::edit::EditHeader;
use libimagrt::setup::generate_runtime_setup;
use libimagstore::storeid::IntoStoreId;
use libimagstore::storeid::StoreIdIterator;
use libimagstore::iter::get::StoreIdGetIteratorExtension;

mod ui;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-edit",
                                    &version,
                                    "Edit store entries with $EDITOR",
                                    ui::build_ui);

    let sids = match rt.cli().value_of("entry") {
        Some(path) => vec![PathBuf::from(path).into_storeid().map_err_trace_exit_unwrap(1)],
        None => if rt.cli().is_present("entries-from-stdin") {
            let stdin = rt.stdin().unwrap_or_else(|| {
                error!("Cannot get handle to stdin");
                ::std::process::exit(1)
            });

            let mut buf = String::new();
            let _ = stdin.lock().read_to_string(&mut buf).unwrap_or_else(|_| {
                error!("Failed to read from stdin");
                ::std::process::exit(1)
            });

            buf.lines()
                .map(PathBuf::from)
                .map(|p| p.into_storeid().map_err_trace_exit_unwrap(1))
                .collect()
        } else {
            error!("Something weird happened. I was not able to find the path of the entries to edit");
            ::std::process::exit(1)
        }
    };

    let edit_header = rt.cli().is_present("edit-header");
    let edit_header_only = rt.cli().is_present("edit-header-only");

    StoreIdIterator::new(Box::new(sids.into_iter().map(Ok)))
        .into_get_iter(rt.store())
        .trace_unwrap_exit(1)
        .map(|o| o.unwrap_or_else(|| {
            error!("Did not find one entry");
            ::std::process::exit(1)
        }))
        .for_each(|mut entry| {
            if edit_header {
                let _ = entry
                    .edit_header_and_content(&rt)
                    .map_err_trace_exit_unwrap(1);
            } else if edit_header_only {
                let _ = entry
                    .edit_header(&rt)
                    .map_err_trace_exit_unwrap(1);
            } else {
                let _ = entry
                    .edit_content(&rt)
                    .map_err_trace_exit_unwrap(1);
            }
        });
}

