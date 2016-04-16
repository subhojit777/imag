#[macro_use] extern crate log;
extern crate clap;
#[macro_use] extern crate semver;
extern crate toml;
extern crate url;
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
use libimaglink::external::ExternalLinker;
use clap::ArgMatches;
use url::Url;

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
                "external" => handle_external_linking(&rt),
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

fn handle_external_linking(rt: &Runtime) {
    use libimagutil::trace::trace_error;

    let scmd       = rt.cli().subcommand_matches("external").unwrap();
    let entry_name = scmd.value_of("id").unwrap(); // enforced by clap
    let entry      = get_entry_by_name(rt, entry_name);
    if entry.is_err() {
        trace_error(&entry.err().unwrap());
        exit(1);
    }
    let mut entry = entry.unwrap();

    if scmd.is_present("add") {
        debug!("Adding link to entry!");
        add_link_to_entry(rt.store(), scmd, &mut entry);
        return;
    }

    if scmd.is_present("remove") {
        debug!("Removing link from entry!");
        remove_link_from_entry(rt.store(), scmd, &mut entry);
        return;
    }

    if scmd.is_present("set") {
        debug!("Setting links in entry!");
        set_links_for_entry(rt.store(), scmd, &mut entry);
        return;
    }

    if scmd.is_present("list") {
        debug!("Listing links in entry!");
        list_links_for_entry(rt.store(), &mut entry);
        return;
    }

    panic!("Clap failed to enforce one of 'add', 'remove', 'set' or 'list'");
}

fn add_link_to_entry(store: &Store, matches: &ArgMatches, entry: &mut FileLockEntry) {
    let link = matches.value_of("add").unwrap();

    let link = Url::parse(link);
    if link.is_err() {
        debug!("URL parsing error...");
        trace_error(&link.err().unwrap());
        debug!("Exiting");
        exit(1);
    }
    let link = link.unwrap();

    if let Err(e) = entry.add_external_link(store, link) {
        debug!("Error while adding external link...");
        trace_error(&e);
    } else {
        debug!("Everything worked well");
        info!("Ok");
    }
}

fn remove_link_from_entry(store: &Store, matches: &ArgMatches, entry: &mut FileLockEntry) {
    let link = matches.value_of("remove").unwrap();

    let link = Url::parse(link);
    if link.is_err() {
        trace_error(&link.err().unwrap());
        exit(1);
    }
    let link = link.unwrap();

    if let Err(e) = entry.remove_external_link(store, link) {
        trace_error(&e);
    } else {
        info!("Ok");
    }
}

fn set_links_for_entry(store: &Store, matches: &ArgMatches, entry: &mut FileLockEntry) {
    let links = matches
        .value_of("links")
        .map(String::from)
        .unwrap()
        .split(",")
        .map(|uri| {
            match Url::parse(uri) {
                Err(e) => {
                    warn!("Could not parse '{}' as URL, ignoring", uri);
                    trace_error(&e);
                    None
                },
                Ok(u) => Some(u),
            }
        })
        .filter_map(|x| x)
        .collect();

    if let Err(e) = entry.set_external_links(store, links) {
        trace_error(&e);
    } else {
        info!("Ok");
    }
}

fn list_links_for_entry(store: &Store, entry: &mut FileLockEntry) {
    let res = entry.get_external_links(store)
        .and_then(|links| {
            let mut i = 0;
            for link in links {
                println!("{: <3}: {}", i, link);
                i += 1;
            }
            Ok(())
        });

    match res {
        Err(e) => {
            trace_error(&e);
        },
        Ok(_) => {
            info!("Ok");
        },
    }
}

