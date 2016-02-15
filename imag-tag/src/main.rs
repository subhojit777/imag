extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagtag;
extern crate libimagutil;

use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagtag::tagable::Tagable;

mod ui;
mod util;

use ui::build_ui;
use util::build_entry_path;

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
            println!("{:?}", rt.err().unwrap());
            exit(1);
        }
    };

    rt.init_logger();

    debug!("Hello. Logging was just enabled");
    debug!("I already set up the Runtime object and build the commandline interface parser.");
    debug!("Lets get rollin' ...");

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
    let path = build_entry_path(rt, id);
    debug!("path = {:?}", path);
    rt.store()
        // "id" must be present, enforced via clap spec
        .retrieve(path)
        .map(|mut e| {
            add.map(|tags| {
                let tags = tags.split(",");
                for tag in tags {
                    info!("Adding tag '{}'", tag);
                    e.add_tag(String::from(tag)).map_err(|e| trace_error(&e));
                }
            });

            rem.map(|tags| {
                let tags = tags.split(",");
                for tag in tags {
                    info!("Removing tag '{}'", tag);
                    e.remove_tag(String::from(tag)).map_err(|e| trace_error(&e));
                }
            });

            set.map(|tags| {
                info!("Setting tags '{}'", tags);
                let tags = tags.split(",").map(String::from).collect();
                e.set_tags(tags);
            });
        })
        .map_err(|e| {
            info!("No entry.");
            debug!("{}", e);
        });
}

fn list(id: &str, rt: &Runtime) {
    let path = build_entry_path(rt, id);
    debug!("path = {:?}", path);

    let entry = rt.store().retrieve(path.clone());
    if entry.is_err() {
        debug!("Could not retrieve '{:?}' => {:?}", id, path);
        warn!("Could not retrieve entry '{}'", id);
        trace_error(&entry.err().unwrap());
        exit(1);
    }
    let entry = entry.unwrap();

    let scmd = rt.cli().subcommand_matches("list").unwrap(); // safe, we checked in main()

    let json_out = scmd.is_present("json");
    let line_out = scmd.is_present("linewise");
    let sepp_out = scmd.is_present("sep");
    let mut comm_out = scmd.is_present("commasep");

    let flags = vec![json_out, line_out, comm_out, sepp_out];

    if flags.iter().filter(|x| **x).count() > 1 {
        // More than one flag passed
        info!("Cannot do more than one thing");
        exit(1);
    }

    if !flags.iter().any(|v| *v) {
        // None of the flags passed, go to default
        comm_out = true;
    }

    let tags = entry.get_tags();
    if tags.is_err() {
        trace_error(&tags.err().unwrap());
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

