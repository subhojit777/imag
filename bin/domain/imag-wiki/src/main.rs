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
extern crate regex;
extern crate filters;
#[macro_use] extern crate log;

#[macro_use] extern crate libimagrt;
extern crate libimagerror;
extern crate libimagstore;
extern crate libimagwiki;
extern crate libimagentryedit;
extern crate libimagentrylink;
extern crate libimagutil;

use std::io::Write;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagerror::iter::TraceIterator;
use libimagerror::trace::MapErrTrace;
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagwiki::store::WikiStore;
use libimagentryedit::edit::{Edit, EditHeader};

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
        Some("ids")         => ids(&rt, wiki_name),
        Some("idof")        => idof(&rt, wiki_name),
        Some("create")      => create(&rt, wiki_name),
        Some("create-wiki") => create_wiki(&rt),
        Some("show")        => show(&rt, wiki_name),
        Some("delete")      => delete(&rt, wiki_name),
        Some(other)         => {
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
        .trace_unwrap_exit(1)
        .for_each(|id| {
            let _ = writeln!(outlock, "{}{}", prefix, id)
                .to_exit_code()
                .unwrap_or_exit();
        });
}

fn idof(rt: &Runtime, wiki_name: &str) {
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
    use libimagwiki::entry::WikiEntry;
    use libimagutil::warn_result::WarnResult;

    let scmd        = rt.cli().subcommand_matches("create").unwrap(); // safed by clap
    let name        = String::from(scmd.value_of("create-name").unwrap()); // safe by clap

    let wiki = rt
        .store()
        .get_wiki(&wiki_name)
        .map_err_trace_exit_unwrap(1)
        .unwrap_or_else(|| {
            error!("No wiki '{}' found", wiki_name);
            ::std::process::exit(1)
        });

    let mut entry = wiki.create_entry(name).map_err_trace_exit_unwrap(1);

    if !scmd.is_present("create-noedit") {
        if scmd.is_present("create-editheader") {
            let _ = entry.edit_header_and_content(rt).map_err_trace_exit_unwrap(1);
        } else {
            let _ = entry.edit_content(rt).map_err_trace_exit_unwrap(1);
        }
    }

    let _ = entry.autolink(rt.store())
        .map_warn_err_str("Linking has failed. Trying to safe the entry now. Please investigate by hand if this succeeds.")
        .map_err(|e| {
            let _ = rt.store().update(&mut entry).map_err_trace_exit_unwrap(1);
            e
        })
        .map_warn_err_str("Safed entry")
        .map_err_trace_exit_unwrap(1);

    if scmd.is_present("create-printid") {
        let out      = rt.stdout();
        let mut lock = out.lock();
        let id       = entry.get_location();

        writeln!(lock, "{}", id).to_exit_code().unwrap_or_exit()
    }
}

fn create_wiki(rt: &Runtime) {
    let scmd      = rt.cli().subcommand_matches("create-wiki").unwrap(); // safed by clap
    let wiki_name = String::from(scmd.value_of("create-wiki-name").unwrap()); // safe by clap
    let _         = rt.store().create_wiki(&wiki_name).map_err_trace_exit_unwrap(1);
}

fn show(rt: &Runtime, wiki_name: &str) {
    use filters::filter::Filter;

    let scmd  = rt.cli().subcommand_matches("show").unwrap(); // safed by clap

    struct NameFilter(Option<Vec<String>>);
    impl Filter<String> for NameFilter {
        fn filter(&self, e: &String) -> bool {
            match self.0 {
                Some(ref v) => v.contains(e),
                None        => false,
            }
        }
    }

    let namefilter = NameFilter(scmd
                                .values_of("show-name")
                                .map(|v| v.map(String::from).collect::<Vec<String>>()));

    let names = scmd
        .values_of("show-name")
        .unwrap() // safe by clap
        .map(String::from)
        .filter(|e| namefilter.filter(e))
        .collect::<Vec<_>>();

    let wiki = rt
        .store()
        .get_wiki(&wiki_name)
        .map_err_trace_exit_unwrap(1)
        .unwrap_or_else(|| {
            error!("No wiki '{}' found", wiki_name);
            ::std::process::exit(1)
        });

    let out         = rt.stdout();
    let mut outlock = out.lock();

    for name in names {
        let entry = wiki
            .get_entry(&name)
            .map_err_trace_exit_unwrap(1)
            .unwrap_or_else(|| {
                error!("No wiki entry '{}' found in wiki '{}'", name, wiki_name);
                ::std::process::exit(1)
            });

        writeln!(outlock, "{}", entry.get_location())
                .to_exit_code()
                .unwrap_or_exit();

        writeln!(outlock, "{}", entry.get_content())
                .to_exit_code()
                .unwrap_or_exit();
    }
}

fn delete(rt: &Runtime, wiki_name: &str) {
    use libimagentrylink::internal::InternalLinker;

    let scmd   = rt.cli().subcommand_matches("delete").unwrap(); // safed by clap
    let name   = String::from(scmd.value_of("delete-name").unwrap()); // safe by clap
    let unlink = !scmd.is_present("delete-no-remove-linkings");

    let wiki = rt
            .store()
            .get_wiki(&wiki_name)
            .map_err_trace_exit_unwrap(1)
            .unwrap_or_else(|| {
                error!("No wiki '{}' found", wiki_name);
                ::std::process::exit(1)
            });

    if unlink {
        wiki.get_entry(&name)
            .map_err_trace_exit_unwrap(1)
            .unwrap_or_else(|| {
                error!("No wiki entry '{}' in '{}' found", name, wiki_name);
                ::std::process::exit(1)
            })
            .unlink(rt.store())
            .map_err_trace_exit_unwrap(1);
    }

    let _ = wiki
        .delete_entry(&name)
        .map_err_trace_exit_unwrap(1);
}

