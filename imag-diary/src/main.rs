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
                "diary" => diary(&rt),
                "view" => view(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            }
        });
}

fn diary(rt: &Runtime) {
    unimplemented!()
}

