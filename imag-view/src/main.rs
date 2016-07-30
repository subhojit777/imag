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
use libimagentryview::builtin::versions::VersionsViewer;
use libimagentryview::viewer::Viewer;

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

    let scmd = match rt.cli().subcommand_matches("view-in") {
        None => {
            debug!("No commandline call");
            exit(1); // we can afford not-executing destructors here
        }
        Some(s) => s,
    };

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

    let res = if rt.cli().is_present("versions") {
        VersionsViewer::new(rt.store()).view_entry(&entry)
    } else {
        if scmd.is_present("view-in-stdout") {
        } else if scmd.is_present("view-in-ui") {
            warn!("Viewing in UI is currently not supported, switch to stdout");
        } else if scmd.is_present("view-in-browser") {
            warn!("Viewing in browser is currently not supported, switch to stdout");
        } else if scmd.is_present("view-in-texteditor") {
            warn!("Viewing in texteditor is currently not supported, switch to stdout");
        } else if scmd.is_present("view-in-custom") {
            warn!("Viewing in custom is currently not supported, switch to stdout");
        }

        StdoutViewer::new(view_header, view_content).view_entry(&entry)
    };

    if let Err(e) = res {
        trace_error_exit(&e, 1);
    }
}

