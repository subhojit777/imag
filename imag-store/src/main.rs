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

mod error;
mod ui;
mod create;
mod retrieve;
mod update;
mod delete;
mod util;

use ui::build_ui;
use create::create;
use retrieve::retrieve;
use update::update;
use delete::delete;

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
                    "create" => create(&rt),
                    "retrieve"   => retrieve(&rt),
                    "update" => update(&rt),
                    "delete" => delete(&rt),
                    _ => {
                        debug!("Unknown command");
                        // More error handling
                    },
                };
            }
        )
}

