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
extern crate filters;
#[macro_use] extern crate nom;
#[macro_use] extern crate log;
#[macro_use] extern crate is_match;
extern crate toml;
extern crate toml_query;

#[cfg(test)]
extern crate env_logger;

extern crate libimagerror;
extern crate libimagstore;
#[macro_use] extern crate libimagrt;

use std::io::Write;
use std::process::exit;

use filters::filter::Filter;

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;
use libimagerror::iter::TraceIterator;
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;

mod id_filters;
mod ui;

use ui::build_ui;
use id_filters::IsInCollectionsFilter;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-ids",
                                    &version,
                                    "print all ids",
                                    build_ui);

    let print_storepath = rt.cli().is_present("print-storepath");

    let values = rt
        .cli()
        .values_of("in-collection-filter")
        .map(|v| v.collect::<Vec<&str>>());

    let collection_filter = IsInCollectionsFilter::new(values);
    let query_filter      : Option<id_filters::header_filter_lang::Query> = rt
        .cli()
        .subcommand_matches("where")
        .map(|matches| {
            let query = matches.value_of("where-filter").unwrap(); // safe by clap
            id_filters::header_filter_lang::parse(&query)
        });

    rt.store()
        .entries()
        .map_err_trace_exit_unwrap(1)
        .trace_unwrap_exit(1)
        .filter(|id| collection_filter.filter(id))
        .filter(|id| match query_filter.as_ref() {
            None     => true,
            Some(qf) => {
                let entry = rt
                    .store()
                    .get(id.clone())
                    .map_err_trace_exit_unwrap(1)
                    .unwrap_or_else(|| {
                        error!("Tried to get '{}', but it does not exist!", id);
                        exit(1)
                    });

                qf.filter(&entry)
            }
        })
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

