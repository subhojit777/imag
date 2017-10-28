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

extern crate clap;
#[macro_use] extern crate log;

#[macro_use] extern crate libimagrt;
extern crate libimagerror;
extern crate libimagstore;
extern crate libimagwiki;

use std::io::Write;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::{MapErrTrace, trace_error};
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagstore::storeid::IntoStoreId;
use libimagwiki::store::WikiStore;

mod ui;
use ui::build_ui;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-wiki",
                                    &version,
                                    "Personal wiki",
                                    build_ui);

    let wiki_name = rt.cli().value_of("wikiname").unwrap_or("default");

    match rt.cli().subcommand_name() {
        Some("ids")    => ids(&rt, wiki_name),
        Some("idof")   => idof(&rt, wiki_name),
        Some("create") => create(&rt, wiki_name),
        Some("delete") => delete(&rt, wiki_name),
        Some("grep")   => grep(&rt, wiki_name),
        Some(other)    => {
            debug!("Unknown command");
            let _ = rt.handle_unknown_subcommand("imag-wiki", other, rt.cli())
                .map_err_trace_exit_unwrap(1)
                .code()
                .map(std::process::exit);
        }
        None => warn!("No command"),
    } // end match scmd
} // end main

fn ids(rt: &Runtime, wiki_name: &str) {
    let scmd   = rt.cli().subcommand_matches("ids").unwrap(); // safed by clap
    let prefix = if scmd.is_present("ids-full") {
        format!("{}/", rt.store().path().display())
    } else {
        String::from("")
    };

    let out         = rt.stdout();
    let mut outlock = out.lock();

    rt.store()
        .get_wiki(wiki_name)
        .map_err_trace_exit_unwrap(1)
        .unwrap_or_else(|| {
            error!("No wiki '{}' found", wiki_name);
            ::std::process::exit(1)
        })
        .all_ids()
        .map_err_trace_exit_unwrap(1)
        .for_each(|id| {
            let _ = writeln!(outlock, "{}{}", prefix, id)
                .to_exit_code()
                .unwrap_or_exit();
        });
}

fn idof(rt: &Runtime, wiki_name: &str) {
    use std::path::PathBuf;
    use libimagstore::storeid::IntoStoreId;

    let scmd = rt.cli().subcommand_matches("idof").unwrap(); // safed by clap

    let entryname = scmd
        .value_of("idof-name")
        .map(String::from)
        .unwrap(); // safed by clap

    let out      = rt.stdout();
    let mut lock = out.lock();

    let _ = rt.store()
        .get_wiki(wiki_name)
        .map_err_trace_exit_unwrap(1)
        .unwrap_or_else(|| {
            error!("No wiki '{}' found", wiki_name);
            ::std::process::exit(1)
        })
        .get_entry(&entryname)
        .map_err_trace_exit_unwrap(1)
        .map(|entry| {
            let id     = entry.get_location().clone();
            let prefix = if scmd.is_present("idof-full") {
                format!("{}/", rt.store().path().display())
            } else {
                String::from("")
            };

            writeln!(lock, "{}{}", prefix, id).to_exit_code().unwrap_or_exit()
        })
        .unwrap_or_else(|| {
            error!("Entry '{}' in wiki '{}' not found!", entryname, wiki_name);
            ::std::process::exit(1)
        });
}

fn create(rt: &Runtime, wiki_name: &str) {
    unimplemented!()
}

fn delete(rt: &Runtime, wiki_name: &str) {
    unimplemented!()
}

fn grep(rt: &Runtime, wiki_name: &str) {
    unimplemented!()
}

