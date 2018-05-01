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

#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimagerror;
extern crate libimagentrylink;

use std::process::exit;

mod ui;
use ui::build_ui;

use std::path::PathBuf;

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;
use libimagerror::iter::TraceIterator;
use libimagstore::storeid::StoreId;
use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::error::StoreError;
use libimagentrylink::internal::InternalLinker;
use libimagstore::iter::get::StoreIdGetIteratorExtension;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-mv",
                                    &version,
                                    "Move things around in the store",
                                    build_ui);

    debug!("mv");

    let sourcename = rt
        .cli()
        .value_of("source")
        .map(PathBuf::from)
        .map(StoreId::new_baseless)
        .unwrap() // unwrap safe by clap
        .map_err_trace_exit_unwrap(1);

    let destname = rt
        .cli()
        .value_of("dest")
        .map(PathBuf::from)
        .map(StoreId::new_baseless)
        .unwrap() // unwrap safe by clap
        .map_err_trace_exit_unwrap(1);

    // remove links to entry, and re-add them later
    let mut linked_entries = {
        rt.store()
            .get(sourcename.clone())
            .map_err_trace_exit_unwrap(1)
            .unwrap_or_else(|| {
                error!("Funny things happened: Entry moved to destination did not fail, but entry does not exist");
                exit(1)
            })
            .get_internal_links()
            .map_err_trace_exit_unwrap(1)
            .map(|link| Ok(link.get_store_id().clone()) as Result<_, StoreError>)
            .into_get_iter(rt.store())
            .trace_unwrap_exit(1)
            .map(|e| {
                e.unwrap_or_else(|| {
                    error!("Linked entry does not exist");
                    exit(1)
                })
            })
            .collect::<Vec<_>>()
    };

    { // remove links to linked entries from source
        let mut entry = rt
            .store()
            .get(sourcename.clone())
            .map_err_trace_exit_unwrap(1)
            .unwrap_or_else(|| {
                error!("Source Entry does not exist");
                exit(1)
            });

        for link in linked_entries.iter_mut() {
            let _ = entry.remove_internal_link(link).map_err_trace_exit_unwrap(1);
        }
    }

    let _ = rt
        .store()
        .move_by_id(sourcename.clone(), destname.clone())
        .map_err(|e| { // on error, re-add links
            debug!("Re-adding links to source entry because moving failed");
            relink(rt.store(), sourcename.clone(), &mut linked_entries);
            e
        })
        .map_err_trace_exit_unwrap(1);

    // re-add links to moved entry
    relink(rt.store(), destname, &mut linked_entries);

    info!("Ok.");
}

fn relink<'a>(store: &'a Store, target: StoreId, linked_entries: &mut Vec<FileLockEntry<'a>>) {
    let mut entry = store
        .get(target)
        .map_err_trace_exit_unwrap(1)
        .unwrap_or_else(|| {
            error!("Funny things happened: Entry moved to destination did not fail, but entry does not exist");
            exit(1)
        });


    for mut link in linked_entries {
        let _ = entry.add_internal_link(&mut link).map_err_trace_exit_unwrap(1);
    }
}
