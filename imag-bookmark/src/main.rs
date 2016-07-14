extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate version;

extern crate libimagbookmark;
extern crate libimagentrylink;
extern crate libimagentrytag;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;

use std::process::exit;

use libimagentrytag::ui::{get_add_tags, get_remove_tags};
use libimagentrylink::internal::Link;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagbookmark::collection::BookmarkCollection;
use libimagbookmark::link::Link as BookmarkLink;
use libimagerror::trace::trace_error;

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
    let coll = scmd.value_of("collection").unwrap(); // enforced by clap

    BookmarkCollection::get(rt.store(), coll)
        .map(|mut collection| {
            for url in scmd.values_of("urls").unwrap() { // enforced by clap
                collection.add_link(BookmarkLink::from(url)).map_err(|e| trace_error(&e));
            }
        });
    info!("Ready");
}

fn collection(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("collection").unwrap();

    if scmd.is_present("add") { // adding a new collection
        let name = scmd.value_of("add").unwrap();
        if let Ok(_) = BookmarkCollection::new(rt.store(), name) {
            info!("Created: {}", name);
        } else {
            warn!("Creating collection {} failed", name);
            exit(1);
        }
    }

    if scmd.is_present("remove") { // remove a collection
        let name = scmd.value_of("remove").unwrap();
        if let Ok(_) = BookmarkCollection::delete(rt.store(), name) {
            info!("Deleted: {}", name);
        } else {
            warn!("Deleting collection {} failed", name);
            exit(1);
        }
    }
}

fn list(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("list").unwrap();
    let coll = scmd.value_of("collection").unwrap(); // enforced by clap

    BookmarkCollection::get(rt.store(), coll)
        .map(|collection| {
            match collection.links() {
                Ok(links) => {
                    debug!("Listing...");
                    for (i, link) in links.iter().enumerate() {
                        println!("{: >3}: {}", i, link);
                    };
                    debug!("... ready with listing");
                },
                Err(e) => {
                    trace_error(&e);
                    exit(1);
                },
            }
        });
    info!("Ready");
}

fn remove(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("remove").unwrap();
    let coll = scmd.value_of("collection").unwrap(); // enforced by clap

    BookmarkCollection::get(rt.store(), coll)
        .map(|mut collection| {
            for url in scmd.values_of("urls").unwrap() { // enforced by clap
                collection.remove_link(BookmarkLink::from(url)).map_err(|e| trace_error(&e));
            }
        });
    info!("Ready");
}

