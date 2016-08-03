use clap::ArgMatches;
use toml::Value;

use libimagstore::store::FileLockEntry;
use libimagstore::storeid::build_entry_path;
use libimagrt::runtime::Runtime;
use libimagerror::trace::{trace_error, trace_error_exit};

pub fn retrieve(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("retrieve")
        .map(|scmd| {
            scmd.value_of("id")
                .map(|id| {
                    let path = try!(build_entry_path(rt.store(), id)
                                    .map_err(|e| trace_error_exit(&e, 1)));
                    debug!("path = {:?}", path);

                    rt.store()
                        // "id" must be present, enforced via clap spec
                        .retrieve(path)
                        .map(|e| print_entry(rt, scmd, e))
                        .map_err(|e| {
                            debug!("No entry.");
                            debug!("{}:", e);
                            trace_error(&e);
                        })
                })
        });
}

pub fn print_entry(rt: &Runtime, scmd: &ArgMatches, e: FileLockEntry) {
    if do_print_raw(scmd) {
        debug!("Printing raw content...");
        println!("{}", e.to_str());
    } else if do_filter(scmd) {
        debug!("Filtering...");
        warn!("Filtering via header specs is currently now supported.");
        warn!("Will fail now!");
        unimplemented!()
    } else {
        debug!("Printing structured...");
        if do_print_header(scmd) {
            debug!("Printing header...");
            if do_print_header_as_json(rt.cli()) {
                debug!("Printing header as json...");
                warn!("Printing as JSON currently not supported.");
                warn!("Will fail now!");
                unimplemented!()
            } else {
                debug!("Printing header as TOML...");
                // We have to Value::Table() for Display
                println!("{}", Value::Table(e.get_header().clone().into()))
            }
        }

        if do_print_content(scmd) {
            debug!("Printing content...");
            println!("{}", e.get_content());
        }

    }
}

fn do_print_header(m: &ArgMatches) -> bool {
    m.is_present("header")
}

fn do_print_header_as_json(m: &ArgMatches) -> bool {
    m.is_present("header-json")
}

fn do_print_content(m: &ArgMatches) -> bool {
    m.is_present("content")
}

fn do_print_raw(m: &ArgMatches) -> bool {
    m.is_present("raw")
}

fn do_filter(m: &ArgMatches) -> bool {
    m.subcommand_matches("filter-header").is_some()
}

