extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagentrytag;
extern crate libimagutil;

use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagentrytag::tagable::Tagable;
use libimagstore::storeid::build_entry_path;

mod ui;

use ui::build_ui;

use libimagutil::trace::trace_error;

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
            println!("{:?}", rt.unwrap_err());
            exit(1);
        }
    };

    let id = rt.cli().value_of("id").unwrap(); // enforced by clap
    rt.cli()
        .subcommand_name()
        .map_or_else(
            || {
                let add = rt.cli().value_of("add");
                let rem = rt.cli().value_of("remove");
                let set = rt.cli().value_of("set");

                alter(&rt, id, add, rem, set);
            },
            |name| {
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

fn alter(rt: &Runtime, id: &str, add: Option<&str>, rem: Option<&str>, set: Option<&str>) {
    let path = {
        match build_entry_path(rt.store(), id) {
            Err(e) => {
                trace_error(&e);
                exit(1);
            },
            Ok(s) => s,
        }
    };
    debug!("path = {:?}", path);

    rt.store()
        // "id" must be present, enforced via clap spec
        .retrieve(path)
        .map(|mut e| {
            add.map(|tags| {
                for tag in tags.split(',') {
                    info!("Adding tag '{}'", tag);
                    if let Err(e) = e.add_tag(String::from(tag)) {
                        trace_error(&e);
                    }
                }
            });

            rem.map(|tags| {
                for tag in tags.split(',') {
                    info!("Removing tag '{}'", tag);
                    if let Err(e) = e.remove_tag(String::from(tag)) {
                        trace_error(&e);
                    }
                }
            });

            set.map(|tags| {
                info!("Setting tags '{}'", tags);
                let tags = tags.split(',').map(String::from).collect();
                if let Err(e) = e.set_tags(tags) {
                    trace_error(&e);
                }
            });
        })
        .map_err(|e| {
            info!("No entry.");
            trace_error(&e);
        })
        .ok();
}

fn list(id: &str, rt: &Runtime) {
    let path = {
        match build_entry_path(rt.store(), id) {
            Err(e) => {
                trace_error(&e);
                exit(1);
            },
            Ok(s) => s,
        }
    };
    debug!("path = {:?}", path);

    let entry = rt.store().retrieve(path.clone());
    if entry.is_err() {
        debug!("Could not retrieve '{:?}' => {:?}", id, path);
        warn!("Could not retrieve entry '{}'", id);
        trace_error(&entry.unwrap_err());
        exit(1);
    }
    let entry = entry.unwrap();

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
        trace_error(&tags.unwrap_err());
        exit(1);
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

