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

use libimagrt::runtime::Runtime;
use std::process::exit;

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
    let name = "imag-store";
    let version = &version!()[..];
    let about = "Direct interface to the store. Use with great care!";
    let ui = build_ui(Runtime::get_default_cli_builder(name, version, about));
    let rt = {
        let rt = Runtime::new(ui);
        if rt.is_ok() {
            rt.unwrap()
        } else {
            println!("Could not set up Runtime");
            println!("{:?}", rt.err().unwrap());
            exit(1);
        }
    };

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

