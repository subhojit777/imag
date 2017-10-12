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

#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate clap;

extern crate libimagrt;
extern crate libimagstore;
extern crate libimagerror;

mod ui;
use ui::build_ui;

use std::path::PathBuf;

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;
use libimagstore::storeid::StoreId;

fn main() {
    let rt = generate_runtime_setup("imag-mv",
                                    &version!()[..],
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

    let _ = rt
        .store()
        .move_by_id(sourcename, destname)
        .map_err_trace_exit(1);

    info!("Ok.");
}
