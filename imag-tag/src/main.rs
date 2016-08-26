extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagentrytag;
extern crate libimagerror;

use std::process::exit;
use std::path::PathBuf;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagentrytag::tagable::Tagable;
use libimagentrytag::tag::Tag;
use libimagerror::trace::{trace_error, trace_error_exit};
use libimagentrytag::ui::{get_add_tags, get_remove_tags};
use libimagstore::storeid::StoreId;

mod ui;

use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-store",
                                    &version!()[..],
                                    "Direct interface to the store. Use with great care!",
                                    build_ui);

    let id = rt.cli().value_of("id").unwrap(); // enforced by clap
    rt.cli()
        .subcommand_name()
        .map_or_else(
            || {
                let id = PathBuf::from(id);
                let add = get_add_tags(rt.cli());
                let rem = get_remove_tags(rt.cli());
                alter(&rt, id, add, rem);
            },
            |name| {
                let id = PathBuf::from(id);
                debug!("Call: {}", name);
                match name {
                    "list" => list(id, &rt),
                    _ => {
                        warn!("Unknown command");
                        // More error handling
                    },
                };
            });
}

fn alter(rt: &Runtime, id: PathBuf, add: Option<Vec<Tag>>, rem: Option<Vec<Tag>>) {
    let path = {
        match StoreId::new(Some(rt.store().path().clone()), id) {
            Err(e) => trace_error_exit(&e, 1),
            Ok(s) => s,
        }
    };
    debug!("path = {:?}", path);

    match rt.store().get(path) {
        Ok(Some(mut e)) => {
            add.map(|tags| {
                for tag in tags {
                    debug!("Adding tag '{:?}'", tag);
                    if let Err(e) = e.add_tag(tag) {
                        trace_error(&e);
                    }
                }
            }); // it is okay to ignore a None here

            rem.map(|tags| {
                for tag in tags {
                    debug!("Removing tag '{:?}'", tag);
                    if let Err(e) = e.remove_tag(tag) {
                        trace_error(&e);
                    }
                }
            }); // it is okay to ignore a None here
        },

        Ok(None) => {
            info!("No entry found.");
        },

        Err(e) => {
            info!("No entry.");
            trace_error(&e);
        },
    }
}

fn list(id: PathBuf, rt: &Runtime) {
    let path = match StoreId::new(Some(rt.store().path().clone()), id) {
        Err(e) => trace_error_exit(&e, 1),
        Ok(s)  => s,
    };
    debug!("path = {:?}", path);

    let entry = match rt.store().get(path.clone()) {
        Ok(Some(e)) => e,
        Ok(None) => {
            info!("No entry found.");
            exit(1);
        },

        Err(e) => {
            warn!("Could not get entry '{:?}'", path);
            trace_error_exit(&e, 1);
        },
    };

    let scmd = rt.cli().subcommand_matches("list").unwrap(); // safe, we checked in main()

    let json_out = scmd.is_present("json");
    let line_out = scmd.is_present("linewise");
    let sepp_out = scmd.is_present("sep");
    let mut comm_out = scmd.is_present("commasep");

    if !vec![json_out, line_out, comm_out, sepp_out].iter().any(|v| *v) {
        // None of the flags passed, go to default
        comm_out = true;
    }

    let tags = entry.get_tags();
    if tags.is_err() {
        trace_error_exit(&tags.unwrap_err(), 1);
    }
    let tags = tags.unwrap();

    if json_out {
        unimplemented!()
    }

    if line_out {
        for tag in &tags {
            println!("{}", tag);
        }
    }

    if sepp_out {
        let sepp = scmd.value_of("sep").unwrap(); // we checked before
        println!("{}", tags.join(sepp));
    }

    if comm_out {
        println!("{}", tags.join(", "));
    }
}

