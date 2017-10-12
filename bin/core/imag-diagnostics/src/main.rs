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
extern crate toml;
extern crate toml_query;
#[macro_use] extern crate version;

extern crate libimagrt;
extern crate libimagerror;
extern crate libimagstore;

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;
use libimagstore::store::FileLockEntry;
use libimagstore::iter::get::*;
use libimagstore::error::StoreError as Error;

use toml::Value;
use toml_query::read::TomlValueReadExt;

use std::collections::BTreeMap;

mod ui;

struct Diagnostic {
    pub entry_store_version: String,
    pub header_sections: usize,
    pub bytecount_content: usize,
    pub overall_byte_size: usize,
    pub verified: bool,
}

impl<'a> From<FileLockEntry<'a>> for Diagnostic {

    fn from(entry: FileLockEntry<'a>) -> Diagnostic {
        Diagnostic {
            entry_store_version: entry
                .get_header()
                .read("imag.version")
                .map(|opt| match opt {
                    Some(&Value::String(ref s)) => s.clone(),
                    Some(_) => "Non-String type in 'imag.version'".to_owned(),
                    None => "No version".to_owned(),
                })
                .unwrap_or("Error reading version".to_owned()),
            header_sections: match entry.get_header() {
                &Value::Table(ref map) => map.keys().count(),
                _ => 0
            },
            bytecount_content: entry.get_content().as_str().len(),
            overall_byte_size: entry.to_str().as_str().len(),
            verified: entry.verify().is_ok(),
        }
    }
}

fn main() {
    let rt = generate_runtime_setup("imag-diagnostics",
                                    &version!()[..],
                                    "Print diagnostics about imag and the imag store",
                                    ui::build_ui);

    let diags = rt.store()
        .entries()
        .map_err_trace_exit(1)
        .unwrap()
        .into_get_iter(rt.store())
        .map(|e| {
            e.map_err_trace_exit_unwrap(1)
                .ok_or(Error::from("Unable to get entry".to_owned()))
                .map_err_trace_exit_unwrap(1)
        })
        .map(Diagnostic::from)
        .collect::<Vec<_>>();

    let mut version_counts        : BTreeMap<String, usize> = BTreeMap::new();
    let mut sum_header_sections   = 0;
    let mut sum_bytecount_content = 0;
    let mut sum_overall_byte_size = 0;
    let mut verified_count        = 0;
    let mut unverified_count      = 0;

    for diag in diags.iter() {
        sum_header_sections     += diag.header_sections;
        sum_bytecount_content   += diag.bytecount_content;
        sum_overall_byte_size   += diag.overall_byte_size;

        let n = version_counts.get(&diag.entry_store_version).map(Clone::clone).unwrap_or(0);
        version_counts.insert(diag.entry_store_version.clone(), n+1);

        if diag.verified {
            verified_count += 1;
        } else {
            unverified_count += 1;
        }
    }

    let n = diags.len();

    println!("imag version {}", version!());
    println!("");
    println!("{} entries", n);
    for (k, v) in version_counts {
        println!("{} entries with store version '{}'", v, k);
    }
    if n != 0 {
        println!("{} header sections in the average entry", sum_header_sections / n);
        println!("{} average content bytecount", sum_bytecount_content / n);
        println!("{} average overall bytecount", sum_overall_byte_size / n);
        println!("{} verified entries", verified_count);
        println!("{} unverified entries", unverified_count);
    }
}

