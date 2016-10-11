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
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimagrt;
extern crate libimagstore;
extern crate libimagentryview;
#[macro_use] extern crate libimagerror;

use std::process::exit;
use std::path::PathBuf;

use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::trace_error_exit;
use libimagentryview::builtin::stdout::StdoutViewer;
use libimagentryview::viewer::Viewer;

mod ui;
mod editor;

use ui::build_ui;
use editor::Editor;

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

    let res = {
        match rt.cli().subcommand_matches("view-in") {
            None => {
                debug!("No commandline call");
                debug!("Assuming to view in cli (stdout)");
            },
            Some(s) => {
                if s.is_present("view-in-stdout") {
                } else if s.is_present("view-in-ui") {
                    warn!("Viewing in UI is currently not supported, switch to stdout");
                } else if s.is_present("view-in-browser") {
                    warn!("Viewing in browser is currently not supported, switch to stdout");
                } else if s.is_present("view-in-texteditor") {
                    if let Err(e) = Editor::new(&rt, &entry).show() {
                        error!("Cannot view in editor: {}", e);
                        trace_error_exit(&e, 1);
                    }
                } else if s.is_present("view-in-custom") {
                    warn!("Viewing in custom is currently not supported, switch to stdout");
                }
            },
        };

        StdoutViewer::new(view_header, view_content).view_entry(&entry)
    };

    if let Err(e) = res {
        trace_error_exit(&e, 1);
    }
}

