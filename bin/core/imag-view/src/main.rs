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
#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate handlebars;
extern crate tempfile;
extern crate toml;
extern crate toml_query;

extern crate libimagentryview;
extern crate libimagerror;
extern crate libimagrt;
extern crate libimagstore;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::exit;

use handlebars::Handlebars;
use toml_query::read::TomlValueReadExt;
use toml::Value;

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::trace_error_exit;
use libimagerror::trace::MapErrTrace;
use libimagentryview::builtin::stdout::StdoutViewer;
use libimagentryview::viewer::Viewer;
use libimagentryview::error::ViewError as VE;

mod ui;
use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup( "imag-view",
                                     &version!()[..],
                                     "View entries (readonly)",
                                     build_ui);

    let entry_id     = rt.cli().value_of("id").unwrap(); // enforced by clap
    let view_header  = rt.cli().is_present("view-header");
    let view_content = rt.cli().is_present("view-content");

    let entry = match rt.store().get(PathBuf::from(entry_id)) {
        Ok(Some(fle)) => fle,
        Ok(None) => {
            error!("Cannot get {}, there is no such id in the store", entry_id);
            exit(1);
        }
        Err(e) => {
            trace_error_exit(&e, 1);
        }
    };

    if rt.cli().is_present("in") {
        let viewer = rt
            .cli()
            .value_of("in")
            .ok_or_else::<VE, _>(|| "No viewer given".to_owned().into())
            .map_err_trace_exit(1)
            .unwrap(); // saved by above call

        let config = rt
            .config()
            .ok_or_else::<VE, _>(|| "No configuration, cannot continue".to_owned().into())
            .map_err_trace_exit(1)
            .unwrap();

        let query = format!("view.viewers.{}", viewer);
        match config.config().read(&query) {
            Err(e) => trace_error_exit(&e, 1),
            Ok(None) => {
                error!("Cannot find '{}' in config", query);
                exit(1)
            },

            Ok(Some(&Value::String(ref viewer_template))) => {
                let mut handlebars = Handlebars::new();
                handlebars.register_escape_fn(::handlebars::no_escape);

                let _ = handlebars.register_template_string("template", viewer_template)
                    .map_err_trace_exit(1)
                    .unwrap();

                let file = {
                    let mut tmpfile = tempfile::NamedTempFile::new()
                        .map_err_trace_exit(1)
                        .unwrap();
                    if view_header {
                        let hdr = toml::ser::to_string_pretty(entry.get_header())
                            .map_err_trace_exit(1)
                            .unwrap();
                        let _ = tmpfile.write(format!("---\n{}---\n", hdr).as_bytes())
                            .map_err_trace_exit(1)
                            .unwrap();
                    }

                    if view_content {
                        let _ = tmpfile.write(entry.get_content().as_bytes())
                            .map_err_trace_exit(1)
                            .unwrap();
                    }

                    tmpfile
                };

                let file_path = file
                    .path()
                    .to_str()
                    .map(String::from)
                    .ok_or::<VE>("Cannot build path".to_owned().into())
                    .map_err_trace_exit(1).unwrap();

                let mut command = {
                    let mut data = BTreeMap::new();
                    data.insert("entry", file_path);

                    let call = handlebars.render("template", &data).map_err_trace_exit(1).unwrap();
                    let mut elems = call.split_whitespace();
                    let command_string = elems
                        .next()
                        .ok_or::<VE>("No command".to_owned().into())
                        .map_err_trace_exit(1)
                        .unwrap();
                    let mut cmd = Command::new(command_string);

                    for arg in elems {
                        cmd.arg(arg);
                    }

                    cmd
                };

                if !command.status().map_err_trace_exit(1).unwrap().success() {
                    exit(1)
                }
            },
            Ok(Some(_)) => {
                error!("Type error: Expected String at {}, found non-string", query);
                exit(1)
            },
        }
    } else {
        let _ = StdoutViewer::new(view_header, view_content)
            .view_entry(&entry)
            .map_err_trace_exit(1);
    }
}

