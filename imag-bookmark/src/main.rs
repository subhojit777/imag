extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate version;

extern crate libimagbookmark;
extern crate libimagentrylink;
extern crate libimagentrytag;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;

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
    unimplemented!()
}

fn list(rt: &Runtime) {
    unimplemented!()
}

fn remove(rt: &Runtime) {
    unimplemented!()
}

