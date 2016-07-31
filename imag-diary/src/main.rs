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

#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate clap;
extern crate chrono;

extern crate libimagdiary;
extern crate libimagentrylist;
extern crate libimagentryview;
extern crate libimaginteraction;
extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;
extern crate libimagtimeui;
#[macro_use] extern crate libimagerror;

use std::process::exit;

use libimagrt::runtime::Runtime;

mod create;
mod delete;
mod edit;
mod list;
mod ui;
mod util;
mod view;

use create::create;
use delete::delete;
use edit::edit;
use list::list;
use ui::build_ui;
use view::view;

fn main() {
    let name = "imag-diary";
    let version = &version!()[..];
    let about = "Personal Diary/Diaries";
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
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "create" => create(&rt),
                "delete" => delete(&rt),
                "edit" => edit(&rt),
                "list" => list(&rt),
                "view" => view(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            }
        });
}

