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
extern crate toml;
extern crate toml_query;

extern crate libimagbookmark;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;

use std::process::exit;

use toml::Value;
use toml_query::read::TomlValueReadExt;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagbookmark::collection::BookmarkCollection;
use libimagbookmark::collection::BookmarkCollectionStore;
use libimagbookmark::error::BookmarkError as BE;
use libimagbookmark::link::Link as BookmarkLink;
use libimagerror::trace::{MapErrTrace, trace_error, trace_error_exit};

mod ui;

use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-bookmark",
                                    &version!()[..],
                                    "Bookmark collection tool",
                                    build_ui);

    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "add"        => add(&rt),
                "collection" => collection(&rt),
                "list"       => list(&rt),
                "remove"     => remove(&rt),
                _            => {
                    debug!("Unknown command"); // More error handling
                },
            }
        });
}

fn add(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("add").unwrap();
    let coll = get_collection_name(rt, "add", "collection");

    let mut collection = BookmarkCollectionStore::get(rt.store(), &coll)
        .map_err_trace_exit_unwrap(1)
        .ok_or(BE::from(format!("No BookmarkcollectionStore '{}' found", coll)))
        .map_err_trace_exit_unwrap(1);

    for url in scmd.values_of("urls").unwrap() { // unwrap saved by clap
        let _ = collection
            .add_link(rt.store(), BookmarkLink::from(url))
            .map_err_trace_exit_unwrap(1);
    }

    info!("Ready");
}

fn collection(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("collection").unwrap();

    if scmd.is_present("add") { // adding a new collection
        let name = scmd.value_of("add").unwrap();
        if let Ok(_) = BookmarkCollectionStore::new(rt.store(), &name) {
            info!("Created: {}", name);
        } else {
            warn!("Creating collection {} failed", name);
            exit(1);
        }
    }

    if scmd.is_present("remove") { // remove a collection
        let name = scmd.value_of("remove").unwrap();
        if let Ok(_) = BookmarkCollectionStore::delete(rt.store(), &name) {
            info!("Deleted: {}", name);
        } else {
            warn!("Deleting collection {} failed", name);
            exit(1);
        }
    }
}

fn list(rt: &Runtime) {
    let coll = get_collection_name(rt, "list", "collection");

    let collection = BookmarkCollectionStore::get(rt.store(), &coll)
        .map_err_trace_exit_unwrap(1)
        .ok_or(BE::from(format!("No BookmarkcollectionStore '{}' found", coll)))
        .map_err_trace_exit_unwrap(1);

    match collection.links(rt.store()) {
        Ok(links) => {
            debug!("Listing...");
            for (i, link) in links.enumerate() {
                match link {
                    Ok(link) => println!("{: >3}: {}", i, link),
                    Err(e)   => trace_error(&e)
                }
            };
            debug!("... ready with listing");
        },
        Err(e) => trace_error_exit(&e, 1),
    }

    info!("Ready");
}

fn remove(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("remove").unwrap();
    let coll = get_collection_name(rt, "list", "collection");

    let mut collection = BookmarkCollectionStore::get(rt.store(), &coll)
        .map_err_trace_exit_unwrap(1)
        .ok_or(BE::from(format!("No BookmarkcollectionStore '{}' found", coll)))
        .map_err_trace_exit_unwrap(1);

    for url in scmd.values_of("urls").unwrap() { // enforced by clap
        collection
            .remove_link(rt.store(), BookmarkLink::from(url))
            .map_err_trace_exit_unwrap(1);
    }

    info!("Ready");
}


fn get_collection_name(rt: &Runtime,
                       subcommand_name: &str,
                       collection_argument_name: &str)
    -> String
{
    rt.cli()
        .subcommand_matches(subcommand_name)
        .and_then(|scmd| scmd.value_of(collection_argument_name).map(String::from))
        .unwrap_or_else(|| {
            rt.config()
                .map(|cfg| match cfg.read("bookmark.default_collection") {
                    Err(e) => trace_error_exit(&e, 1),
                    Ok(Some(&Value::String(ref name))) => name.clone(),
                    Ok(None) => {
                        error!("Missing config: 'bookmark.default_collection'. Set or use commandline to specify.");
                        exit(1)
                    },

                    Ok(Some(_)) => {
                        error!("Type error in configuration: 'bookmark.default_collection' should be string");
                        exit(1)
                    }

                })
                .unwrap_or_else(|| {
                    error!("Failed to read configuration");
                    exit(1)
                })
        })
}

