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
extern crate libimagrt;
extern crate libimagstore;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::exit;

use handlebars::Handlebars;
use toml_query::read::TomlValueReadTypeExt;

use libimagrt::setup::generate_runtime_setup;
use libimagerror::str::ErrFromStr;
use libimagerror::trace::MapErrTrace;
use libimagentryview::builtin::stdout::StdoutViewer;
use libimagentryview::viewer::Viewer;
use libimagentryview::error::ViewError as VE;

mod ui;
use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup( "imag-view",
                                     env!("CARGO_PKG_VERSION"),
                                     "View entries (readonly)",
                                     build_ui);

    let entry_id     = rt.cli().value_of("id").unwrap(); // enforced by clap
    let view_header  = rt.cli().is_present("view-header");
    let hide_content = rt.cli().is_present("not-view-content");

    let entry = match rt.store().get(PathBuf::from(entry_id)).map_err_trace_exit_unwrap(1) {
        Some(fle) => fle,
        None => {
            error!("Cannot get {}, there is no such id in the store", entry_id);
            exit(1);
        }
    };

    if rt.cli().is_present("in") {
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

        let file = {
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

            tmpfile
        };

        let file_path = file
            .path()
            .to_str()
            .map(String::from)
            .ok_or::<VE>("Cannot build path".to_owned().into())
            .map_err_trace_exit_unwrap(1);

        let mut command = {
            let mut data = BTreeMap::new();
            data.insert("entry", file_path);

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

        if !command
            .status()
            .err_from_str()
            .map_err(VE::from)
            .map_err_trace_exit_unwrap(1)
            .success()
        {
            exit(1)
        }
    } else {
        let _ = StdoutViewer::new(view_header, !hide_content)
            .view_entry(&entry)
            .map_err_trace_exit_unwrap(1);
    }
}

