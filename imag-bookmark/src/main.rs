extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate version;

extern crate libimagbookmark;
extern crate libimagentrylink;
extern crate libimagrt;
extern crate libimagutil;

use libimagentrytag::ui::{get_add_tags, get_remove_tags};
use libimagentrylink::internal::Link;

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
    unimplemented!()
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

