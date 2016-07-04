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
extern crate libimagutil;
#[macro_use] extern crate libimagerror;

use libimagrt::setup::generate_runtime_setup;

mod create;
mod delete;
mod error;
mod get;
mod retrieve;
mod ui;
mod update;
mod util;

use create::create;
use delete::delete;
use get::get;
use retrieve::retrieve;
use ui::build_ui;
use update::update;

fn main() {
    let rt = generate_runtime_setup("imag-store",
                                    &version!()[..],
                                    "Direct interface to the store. Use with great care!",
                                    build_ui);

    rt.cli()
        .subcommand_name()
        .map_or_else(
            || {
                debug!("No command");
                // More error handling
            },
            |name| {
                debug!("Call: {}", name);
                match name {
                    "create"   => create(&rt),
                    "delete"   => delete(&rt),
                    "get"      => get(&rt),
                    "retrieve" => retrieve(&rt),
                    "update"   => update(&rt),
                    _ => {
                        debug!("Unknown command");
                        // More error handling
                    },
                };
            }
        )
}

