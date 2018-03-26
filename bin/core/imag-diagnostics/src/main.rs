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
extern crate toml;
extern crate toml_query;

#[macro_use] extern crate libimagrt;
extern crate libimagerror;
extern crate libimagentrylink;
extern crate libimagstore;

use std::io::Write;

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;
use libimagerror::io::ToExitCode;
use libimagerror::exit::ExitUnwrap;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagstore::error::StoreError as Error;
use libimagentrylink::internal::*;

use toml::Value;
use toml_query::read::TomlValueReadExt;

use std::collections::BTreeMap;

mod ui;

struct Diagnostic {
    pub id: StoreId,
    pub entry_store_version: String,
    pub header_sections: usize,
    pub bytecount_content: usize,
    pub overall_byte_size: usize,
    pub verified: bool,
    pub num_internal_links: usize,
}

impl Diagnostic {

    fn for_entry<'a>(entry: FileLockEntry<'a>) -> Result<Diagnostic, ::libimagstore::error::StoreError> {
        Ok(Diagnostic {
            id: entry.get_location().clone(),
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
            overall_byte_size: entry.to_str()?.as_str().len(),
            verified: entry.verify().is_ok(),
            num_internal_links: entry.get_internal_links().map(Iterator::count).unwrap_or(0),
        })
    }
}

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-diagnostics",
                                    &version,
                                    "Print diagnostics about imag and the imag store",
                                    ui::build_ui);

    let diags = rt.store()
        .entries()
        .map_err_trace_exit_unwrap(1)
        .into_get_iter()
        .map(|e| {
            e.map_err_trace_exit_unwrap(1)
                .ok_or(Error::from("Unable to get entry".to_owned()))
                .map_err_trace_exit_unwrap(1)
        })
        .map(Diagnostic::for_entry)
        .collect::<Result<Vec<_>, _>>()
        .map_err_trace_exit_unwrap(1);

    let mut version_counts        : BTreeMap<String, usize> = BTreeMap::new();
    let mut sum_header_sections   = 0;
    let mut sum_bytecount_content = 0;
    let mut sum_overall_byte_size = 0;
    let mut max_overall_byte_size : Option<(usize, StoreId)> = None;
    let mut verified_count        = 0;
    let mut unverified_count      = 0;
    let mut num_internal_links    = 0;
    let mut max_internal_links : Option<(usize, StoreId)> = None;

    for diag in diags.iter() {
        sum_header_sections     += diag.header_sections;
        sum_bytecount_content   += diag.bytecount_content;
        sum_overall_byte_size   += diag.overall_byte_size;
        match max_overall_byte_size {
            None => max_overall_byte_size = Some((diag.num_internal_links, diag.id.clone())),
            Some((num, _)) => if num < diag.overall_byte_size {
                max_overall_byte_size = Some((diag.overall_byte_size, diag.id.clone()));
            }
        }

        let n = version_counts.get(&diag.entry_store_version).map(Clone::clone).unwrap_or(0);
        version_counts.insert(diag.entry_store_version.clone(), n+1);

        if diag.verified {
            verified_count += 1;
        } else {
            unverified_count += 1;
        }

        num_internal_links += diag.num_internal_links;
        match max_internal_links {
            None => max_internal_links = Some((diag.num_internal_links, diag.id.clone())),
            Some((num, _)) => if num < diag.num_internal_links {
                max_internal_links = Some((diag.num_internal_links, diag.id.clone()));
            }
        }
    }

    let n = diags.len();

    let mut out = rt.stdout();

    let _ = writeln!(out, "imag version {}", env!("CARGO_PKG_VERSION"))
        .to_exit_code()
        .unwrap_or_exit();
    let _ = writeln!(out, "")
        .to_exit_code()
        .unwrap_or_exit();
    let _ = writeln!(out, "{} entries", n)
        .to_exit_code()
        .unwrap_or_exit();
    for (k, v) in version_counts {
        let _ = writeln!(out, "{} entries with store version '{}'", v, k)
            .to_exit_code()
            .unwrap_or_exit();
    }
    if n != 0 {
        let _ = writeln!(out, "{} header sections in the average entry", sum_header_sections / n)
            .to_exit_code()
            .unwrap_or_exit();
        let _ = writeln!(out, "{} average content bytecount", sum_bytecount_content / n)
            .to_exit_code()
            .unwrap_or_exit();
        let _ = writeln!(out, "{} average overall bytecount", sum_overall_byte_size / n)
            .to_exit_code()
            .unwrap_or_exit();
        if let Some((num, path)) = max_overall_byte_size {
            let _ = writeln!(out,
                             "Largest Entry ({bytes} bytes): {path}",
                             bytes = num,
                             path = path
                                 .into_pathbuf()
                                 .map_err_trace_exit_unwrap(1)
                                 .to_str()
                                 .unwrap_or("Failed converting path to string")
                )
                .to_exit_code()
                .unwrap_or_exit();
        }
        let _ = writeln!(out, "{} average internal link count per entry", num_internal_links / n)
            .to_exit_code()
            .unwrap_or_exit();
        if let Some((num, path)) = max_internal_links {
            let _ = writeln!(out, "Entry with most internal links ({count}): {path}",
                     count = num,
                     path = path
                        .into_pathbuf()
                        .map_err_trace_exit_unwrap(1)
                        .to_str()
                        .unwrap_or("Failed converting path to string")
            )
            .to_exit_code()
            .unwrap_or_exit();
        }
        let _ = writeln!(out, "{} verified entries", verified_count)
            .to_exit_code()
            .unwrap_or_exit();
        let _ = writeln!(out, "{} unverified entries", unverified_count)
            .to_exit_code()
            .unwrap_or_exit();
    }
}

