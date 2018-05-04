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
#[macro_use]
extern crate log;

extern crate libimagentrycategory;
extern crate libimagerror;
#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimaginteraction;

use libimagerror::trace::MapErrTrace;
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagstore::storeid::IntoStoreId;

mod ui;

use std::io::Write;
use std::io::Read;
use std::path::PathBuf;

use libimagentrycategory::store::CategoryStore;
use libimagstore::storeid::StoreIdIterator;
use libimagstore::iter::get::StoreIdGetIteratorExtension;
use libimagerror::iter::TraceIterator;
use libimagentrycategory::entry::EntryCategory;
use libimagentrycategory::category::Category;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-category",
                                    &version,
                                    "Add a category to entries and manage categories",
                                    ui::build_ui);

    rt.cli()
        .subcommand_name()
        .map(|name| {
            match name {
                "set"               => set(&rt),
                "get"               => get(&rt),
                "list-category"     => list_category(&rt),
                "create-category"   => create_category(&rt),
                "delete-category"   => delete_category(&rt),
                "list-categories"   => list_categories(&rt),
                other               => {
                    debug!("Unknown command");
                    let _ = rt.handle_unknown_subcommand("imag-category", other, rt.cli())
                        .map_err_trace_exit_unwrap(1)
                        .code()
                        .map(::std::process::exit);
                },
            }
        });
}

fn set(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("set").unwrap(); // safed by main()
    let name = scmd.value_of("set-name").map(String::from).unwrap(); // safed by clap
    let sids = match scmd.value_of("set-ids") {
        Some(path) => vec![PathBuf::from(path).into_storeid().map_err_trace_exit_unwrap(1)],
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
                .map(|p| p.into_storeid().map_err_trace_exit_unwrap(1))
                .collect()
        } else {
            error!("Something weird happened. I was not able to find the path of the entries to edit");
            ::std::process::exit(1)
        }
    };

    StoreIdIterator::new(Box::new(sids.into_iter().map(Ok)))
        .into_get_iter(rt.store())
        .trace_unwrap_exit(1)
        .map(|o| o.unwrap_or_else(|| {
            error!("Did not find one entry");
            ::std::process::exit(1)
        }))
        .for_each(|mut entry| {
            let _ = entry
                .set_category_checked(rt.store(), &name)
                .map_err_trace_exit_unwrap(1);
        })
}

fn get(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("get").unwrap(); // safed by main()
    let sids = match scmd.value_of("get-ids") {
        Some(path) => vec![PathBuf::from(path).into_storeid().map_err_trace_exit_unwrap(1)],
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
                .map(|p| p.into_storeid().map_err_trace_exit_unwrap(1))
                .collect()
        } else {
            error!("Something weird happened. I was not able to find the path of the entries to edit");
            ::std::process::exit(1)
        }
    };

    let out         = rt.stdout();
    let mut outlock = out.lock();

    StoreIdIterator::new(Box::new(sids.into_iter().map(Ok)))
        .into_get_iter(rt.store())
        .trace_unwrap_exit(1)
        .map(|o| o.unwrap_or_else(|| {
            error!("Did not find one entry");
            ::std::process::exit(1)
        }))
        .map(|entry| entry.get_category().map_err_trace_exit_unwrap(1))
        .for_each(|name| {
            let _ = writeln!(outlock, "{}", name).to_exit_code().unwrap_or_exit();
        })
}

fn list_category(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("list-category").unwrap(); // safed by main()
    let name = scmd.value_of("list-category-name").map(String::from).unwrap(); // safed by clap

    if let Some(category) = rt.store().get_category_by_name(&name).map_err_trace_exit_unwrap(1) {
        let out         = rt.stdout();
        let mut outlock = out.lock();

        category
            .get_entries(rt.store())
            .map_err_trace_exit_unwrap(1)
            .for_each(|entry| {
                writeln!(outlock, "{}", entry.map_err_trace_exit_unwrap(1).get_location())
                    .to_exit_code()
                    .unwrap_or_exit();
            })
    } else {
        info!("No category named '{}'", name);
        ::std::process::exit(1)
    }
}

fn create_category(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("create-category").unwrap(); // safed by main()
    let name = scmd.value_of("create-category-name").map(String::from).unwrap(); // safed by clap

    let _ = rt
        .store()
        .create_category(&name)
        .map_err_trace_exit_unwrap(1);
}

fn delete_category(rt: &Runtime) {
    use libimaginteraction::ask::ask_bool;

    let scmd   = rt.cli().subcommand_matches("delete-category").unwrap(); // safed by main()
    let name   = scmd.value_of("delete-category-name").map(String::from).unwrap(); // safed by clap
    let ques   = format!("Do you really want to delete category '{}' and remove links to all categorized enties?", name);
    let answer = ask_bool(&ques, Some(false));

    if answer {
        info!("Deleting category '{}'", name);
        let _ = rt
            .store()
            .delete_category(&name)
            .map_err_trace_exit_unwrap(1);
    } else {
        info!("Not doing anything");
    }
}

fn list_categories(rt: &Runtime) {
    let out         = rt.stdout();
    let mut outlock = out.lock();

    rt.store()
        .all_category_names()
        .map_err_trace_exit_unwrap(1)
        .for_each(|name| {
            writeln!(outlock, "{}", name.map_err_trace_exit_unwrap(1))
                .to_exit_code()
                .unwrap_or_exit();
        })
}

