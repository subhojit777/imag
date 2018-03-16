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

extern crate libimagerror;
extern crate libimagstore;
#[macro_use] extern crate libimagrt;

use std::io::Write;

use clap::{Arg, App};

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;


/// No special CLI
pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .arg(Arg::with_name("print-storepath")
             .long("with-storepath")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Print the storepath for each id"))
}

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-ids",
                                    &version,
                                    "print all ids",
                                    build_ui);

    let print_storepath = rt.cli().is_present("print-storepath");

    rt.store()
        .entries()
        .map_err_trace_exit_unwrap(1)
        .map(|id| if print_storepath {
            id
        } else {
            id.without_base()
        })
        .for_each(|id| {
            let _ = writeln!(rt.stdout(), "{}", id.to_str().map_err_trace_exit_unwrap(1))
                .to_exit_code()
                .unwrap_or_exit();
        })
}

