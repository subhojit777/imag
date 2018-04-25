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
extern crate handlebars;
extern crate tempfile;
extern crate toml;
extern crate toml_query;

extern crate libimagentryview;
extern crate libimagerror;
#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;

use std::str::FromStr;
use std::collections::BTreeMap;
use std::io::Write;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::process::exit;

use handlebars::Handlebars;
use toml_query::read::TomlValueReadTypeExt;

use libimagrt::setup::generate_runtime_setup;
use libimagrt::runtime::Runtime;
use libimagerror::str::ErrFromStr;
use libimagerror::trace::MapErrTrace;
use libimagerror::iter::TraceIterator;
use libimagerror::io::ToExitCode;
use libimagerror::exit::ExitUnwrap;
use libimagentryview::builtin::stdout::StdoutViewer;
use libimagentryview::builtin::md::MarkdownViewer;
use libimagentryview::viewer::Viewer;
use libimagentryview::error::ViewError as VE;
use libimagstore::storeid::IntoStoreId;
use libimagstore::error::StoreError;
use libimagstore::iter::get::StoreIdGetIteratorExtension;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;

mod ui;
use ui::build_ui;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup( "imag-view",
                                     &version,
                                     "View entries (readonly)",
                                     build_ui);

    let entry_ids    = entry_ids(&rt);
    let view_header  = rt.cli().is_present("view-header");
    let hide_content = rt.cli().is_present("not-view-content");

    if rt.cli().is_present("in") {
        let files = entry_ids
            .into_iter()
            .into_get_iter(rt.store())
            .map(|e| {
                 e.map_err_trace_exit_unwrap(1)
                     .ok_or_else(|| String::from("Entry not found"))
                     .map_err(StoreError::from)
                     .map_err_trace_exit_unwrap(1)
            })
            .map(|entry| create_tempfile_for(&entry, view_header, hide_content))
            .collect::<Vec<_>>();

        let mut command = {
            let viewer = rt
                .cli()
                .value_of("in")
                .ok_or_else::<VE, _>(|| "No viewer given".to_owned().into())
                .map_err_trace_exit_unwrap(1);

            let config = rt
                .config()
                .ok_or_else::<VE, _>(|| "No configuration, cannot continue".to_owned().into())
                .map_err_trace_exit_unwrap(1);

            let query = format!("view.viewers.{}", viewer);

            let viewer_template = config
                .read_string(&query)
                .map_err_trace_exit_unwrap(1)
                .unwrap_or_else(|| {
                    error!("Cannot find '{}' in config", query);
                    exit(1)
                });

            let mut handlebars = Handlebars::new();
            handlebars.register_escape_fn(::handlebars::no_escape);

            let _ = handlebars
                .register_template_string("template", viewer_template)
                .err_from_str()
                .map_err(VE::from)
                .map_err_trace_exit_unwrap(1);

            let mut data = BTreeMap::new();

            let file_paths = files
                .iter()
                .map(|&(_, ref path)| path.clone())
                .collect::<Vec<String>>()
                .join(" ");

            data.insert("entries", file_paths);

            let call = handlebars
                .render("template", &data)
                .err_from_str()
                .map_err(VE::from)
                .map_err_trace_exit_unwrap(1);
            let mut elems = call.split_whitespace();
            let command_string = elems
                .next()
                .ok_or::<VE>("No command".to_owned().into())
                .map_err_trace_exit_unwrap(1);
            let mut cmd = Command::new(command_string);

            for arg in elems {
                cmd.arg(arg);
            }

            cmd
        };

        debug!("Calling: {:?}", command);

        if !command
            .status()
            .err_from_str()
            .map_err(VE::from)
            .map_err_trace_exit_unwrap(1)
            .success()
        {
            exit(1)
        }

        drop(files);
    } else {
        let iter = entry_ids
            .into_iter()
            .into_get_iter(rt.store())
            .map(|e| {
                 e.map_err_trace_exit_unwrap(1)
                     .ok_or_else(|| String::from("Entry not found"))
                     .map_err(StoreError::from)
                     .map_err_trace_exit_unwrap(1)
            });

        let out         = rt.stdout();
        let mut outlock = out.lock();

        let basesep = if rt.cli().occurrences_of("seperator") != 0 { // checker for default value
            rt.cli().value_of("seperator").map(String::from)
        } else {
            None
        };

        let mut sep_width = 80; // base width, automatically overridden by wrap width

        // Helper to build the seperator with a base string `sep` and a `width`
        let build_seperator = |sep: String, width: usize| -> String {
            sep.repeat(width / sep.len())
        };

        if rt.cli().is_present("compile-md") {
            let viewer    = MarkdownViewer::new(&rt);
            let seperator = basesep.map(|s| build_seperator(s, sep_width));

            for (n, entry) in iter.enumerate() {
                if n != 0 {
                    seperator
                        .as_ref()
                        .map(|s| writeln!(outlock, "{}", s).to_exit_code().unwrap_or_exit());
                }

                viewer
                    .view_entry(&entry, &mut outlock)
                    .map_err_trace_exit_unwrap(1);
            }
        } else {
            let mut viewer = StdoutViewer::new(view_header, !hide_content);

            if rt.cli().occurrences_of("autowrap") != 0 {
                let width = rt.cli().value_of("autowrap").unwrap(); // ensured by clap
                let width = usize::from_str(width).unwrap_or_else(|e| {
                    error!("Failed to parse argument to number: autowrap = {:?}",
                           rt.cli().value_of("autowrap").map(String::from));
                    error!("-> {:?}", e);
                    ::std::process::exit(1)
                });

                // Copying this value over, so that the seperator has the right len as well
                sep_width = width;

                viewer.wrap_at(width);
            }

            let seperator = basesep.map(|s| build_seperator(s, sep_width));
            for (n, entry) in iter.enumerate() {
                if n != 0 {
                    seperator
                        .as_ref()
                        .map(|s| writeln!(outlock, "{}", s).to_exit_code().unwrap_or_exit());
                }

                viewer
                    .view_entry(&entry, &mut outlock)
                    .map_err_trace_exit_unwrap(1);
            }
        }
    }
}

fn entry_ids(rt: &Runtime) -> Vec<StoreId> {
    match rt.cli().values_of("id") {
        Some(pathes) => pathes
            .map(PathBuf::from)
            .map(PathBuf::into_storeid)
            .trace_unwrap_exit(1)
            .collect(),

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
                .map(PathBuf::into_storeid)
                .trace_unwrap_exit(1)
                .collect()
        } else {
            error!("Something weird happened. I was not able to find the path of the entries to edit");
            ::std::process::exit(1)
        }
    }
}

fn create_tempfile_for<'a>(entry: &FileLockEntry<'a>, view_header: bool, hide_content: bool)
    -> (tempfile::NamedTempFile, String)
{
    let mut tmpfile = tempfile::NamedTempFile::new()
        .err_from_str()
        .map_err(VE::from)
        .map_err_trace_exit_unwrap(1);

    if view_header {
        let hdr = toml::ser::to_string_pretty(entry.get_header())
            .err_from_str()
            .map_err(VE::from)
            .map_err_trace_exit_unwrap(1);
        let _ = tmpfile.write(format!("---\n{}---\n", hdr).as_bytes())
            .err_from_str()
            .map_err(VE::from)
            .map_err_trace_exit_unwrap(1);
    }

    if !hide_content {
        let _ = tmpfile.write(entry.get_content().as_bytes())
            .err_from_str()
            .map_err(VE::from)
            .map_err_trace_exit_unwrap(1);
    }

    let file_path = tmpfile
        .path()
        .to_str()
        .map(String::from)
        .ok_or::<VE>("Cannot build path".to_owned().into())
        .map_err_trace_exit_unwrap(1);

    (tmpfile, file_path)
}

