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
extern crate regex;
#[macro_use] extern crate version;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagerror;

use regex::Regex;

use libimagrt::setup::generate_runtime_setup;
use libimagstore::iter::get::StoreIdGetIteratorExtension;
use libimagstore::store::Entry;
use libimagerror::trace::MapErrTrace;

mod ui;

struct Options {
    files_with_matches: bool,
    count: bool,
}

fn main() {
    let rt = generate_runtime_setup("imag-grep",
                                    &version!()[..],
                                    "grep through entries text",
                                    ui::build_ui);

    let opts = Options {
        files_with_matches    : rt.cli().is_present("files-with-matches"),
        count                 : rt.cli().is_present("count"),
    };

    let mut count : usize = 0;

    let pattern = rt
        .cli()
        .value_of("pattern")
        .map(Regex::new)
        .unwrap() // ensured by clap
        .map_err_trace_exit_unwrap(1);

    let overall_count = rt
        .store()
        .entries()
        .map_err_trace_exit_unwrap(1)
        .into_get_iter(rt.store())
        .filter_map(|res| res.map_err_trace_exit_unwrap(1))
        .filter(|entry| pattern.is_match(entry.get_content()))
        .map(|entry| show(&entry, &pattern, &opts, &mut count))
        .count();

    if opts.count {
        println!("{}", count);
    } else {
        println!("Processed {} files, {} matches, {} nonmatches",
                 overall_count,
                 count,
                 overall_count - count);
    }
}

fn show(e: &Entry, re: &Regex, opts: &Options, count: &mut usize) {
    if opts.files_with_matches {
        println!("{}", e.get_location());
    } else if opts.count {
        *count += 1;
    } else {
        println!("{}:", e.get_location());
        for capture in re.captures_iter(e.get_content()) {
            for mtch in capture.iter() {
                if let Some(m) = mtch {
                    println!(" '{}'", m.as_str());
                }
            }
        }

        println!("");
    }
}

