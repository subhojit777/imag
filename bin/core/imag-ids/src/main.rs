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

extern crate libimagerror;
extern crate libimagstore;
#[macro_use] extern crate libimagrt;

use std::io::Write;

use filters::filter::Filter;
use clap::{Arg, App};

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagstore::storeid::StoreId;


pub struct IsInCollectionsFilter<'a, A>(Option<A>, ::std::marker::PhantomData<&'a str>)
    where A: AsRef<[&'a str]>;

impl<'a, A> Filter<StoreId> for IsInCollectionsFilter<'a, A>
    where A: AsRef<[&'a str]> + 'a
{
    fn filter(&self, sid: &StoreId) -> bool {
        match self.0 {
            Some(ref colls) => sid.is_in_collection(colls),
            None => true,
        }
    }
}

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

    let collection_filter = IsInCollectionsFilter(values, ::std::marker::PhantomData);

    rt.store()
        .entries()
        .map_err_trace_exit_unwrap(1)
        .filter(|id| collection_filter.filter(id))
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

