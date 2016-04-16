#[macro_use] extern crate log;
extern crate clap;
#[macro_use] extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimaglink;
extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;

use std::process::exit;
use std::ops::Deref;
use std::error::Error;

use libimagrt::runtime::Runtime;
use libimagstore::error::StoreError;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagutil::trace::trace_error;

mod ui;

use ui::build_ui;

fn main() {
    let name = "imag-link";
    let version = &version!()[..];
    let about = "Link entries";
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

    debug!("Hello. Logging was just enabled");
    debug!("I already set up the Runtime object and build the commandline interface parser.");
    debug!("Lets get rollin' ...");

    rt.cli()
        .subcommand_name()
        .map(|name| {
            match name {
                "internal" => handle_internal_linking(&rt),
                "external" => { unimplemented!() },
                _ => {
                    warn!("No commandline call");
                    exit(1);
                },
            }
        });
}

fn handle_internal_linking(rt: &Runtime) {
    use libimaglink::internal::InternalLinker;
    use libimagutil::trace::trace_error;

    debug!("Handle internal linking call");
    let cmd = rt.cli().subcommand_matches("internal").unwrap();

    if cmd.is_present("list") {
        debug!("List...");
        for entry in cmd.value_of("list").unwrap().split(",") {
            debug!("Listing for '{}'", entry);
            match get_entry_by_name(rt, entry) {
                Ok(e) => {
                    e.get_internal_links()
                        .map(|links| {
                            let mut i = 0;
                            for link in links.iter().map(|l| l.to_str()).filter_map(|x| x) {
                                println!("{: <3}: {}", i, link);
                                i += 1;
                            }
                        });
                },

                Err(e) => {
                    trace_error(&e);
                    break;
                },
            }
        }
        debug!("Listing ready!");
    } else {
        let mut from = {
            let mut from = get_from_entry(&rt);
            if from.is_none() {
                warn!("No 'from' entry");
                exit(1);
            }
            from.unwrap()
        };
        debug!("Link from = {:?}", from.deref());

        let mut to = {
            let mut to = get_to_entries(&rt);
            if to.is_none() {
                warn!("No 'to' entry");
                exit(1);
            }
            to.unwrap()
        };
        debug!("Link to = {:?}", to.iter().map(|f| f.deref()).collect::<Vec<&Entry>>());

        match cmd.subcommand_name() {
            Some("add") => {
                for mut to_entry in to {
                    if let Err(e) = to_entry.add_internal_link(&mut from) {
                        trace_error(&e);
                        exit(1);
                    }
                }
            },

            Some("remove") => {
                for mut to_entry in to {
                    if let Err(e) = to_entry.remove_internal_link(&mut from) {
                        trace_error(&e);
                        exit(1);
                    }
                }
            },

            _ => unreachable!(),
        };
    }
}

fn get_from_entry<'a>(rt: &'a Runtime) -> Option<FileLockEntry<'a>> {
    rt.cli()
        .subcommand_matches("internal")
        .unwrap() // safe, we know there is an "internal" subcommand"
        .subcommand_matches("add")
        .unwrap() // safe, we know there is an "add" subcommand
        .value_of("from")
        .and_then(|from_name| {
            match get_entry_by_name(rt, from_name) {
                Err(e) => {
                    debug!("We couldn't get the entry from name: '{:?}'", from_name);
                    trace_error(&e); None
                },
                Ok(e) => Some(e),
            }

        })
}

fn get_to_entries<'a>(rt: &'a Runtime) -> Option<Vec<FileLockEntry<'a>>> {
    rt.cli()
        .subcommand_matches("internal")
        .unwrap() // safe, we know there is an "internal" subcommand"
        .subcommand_matches("add")
        .unwrap() // safe, we know there is an "add" subcommand
        .values_of("to")
        .map(|values| {
            let mut v = vec![];
            for entry in values.map(|v| get_entry_by_name(rt, v)) {
                match entry {
                    Err(e) => trace_error(&e),
                    Ok(e) => v.push(e),
                }
            }
            v
        })
}

fn get_entry_by_name<'a>(rt: &'a Runtime, name: &str) -> Result<FileLockEntry<'a>, StoreError> {
    use libimagstore::storeid::build_entry_path;
    build_entry_path(rt.store(), name)
        .and_then(|path| rt.store().retrieve(path))
}

