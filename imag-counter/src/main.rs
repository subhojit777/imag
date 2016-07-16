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

extern crate libimagcounter;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;

use std::process::exit;
use std::str::FromStr;

use libimagrt::setup::generate_runtime_setup;
use libimagcounter::counter::Counter;
use libimagerror::trace::{trace_error, trace_error_exit};
use libimagutil::key_value_split::IntoKeyValue;

mod create;
mod delete;
mod interactive;
mod list;
mod ui;

use ui::build_ui;
use create::create;
use delete::delete;
use interactive::interactive;
use list::list;

enum Action {
    Inc,
    Dec,
    Reset,
    Set,
}

fn main() {
    let rt = generate_runtime_setup("imag-counter",
                                    &version!()[..],
                                    "Counter tool to count things",
                                    build_ui);

    rt.cli()
        .subcommand_name()
        .map_or_else(|| {
                let (action, name) = {
                    if rt.cli().is_present("increment") {
                        (Action::Inc, rt.cli().value_of("increment").unwrap())
                    } else if rt.cli().is_present("decrement") {
                        (Action::Dec, rt.cli().value_of("decrement").unwrap())
                    } else if rt.cli().is_present("reset") {
                        (Action::Reset, rt.cli().value_of("reset").unwrap())
                    } else /* rt.cli().is_present("set") */ {
                        (Action::Set, rt.cli().value_of("set").unwrap())
                    }
                };

                match action {
                    Action::Inc => {
                        Counter::load(String::from(name), rt.store())
                            .map(|mut counter| {
                                match counter.inc() {
                                    Err(e) => trace_error_exit(&e, 1),
                                    Ok(_) => info!("Ok"),
                                }
                            })
                    },
                    Action::Dec => {
                        Counter::load(String::from(name), rt.store())
                            .map(|mut counter| {
                                match counter.dec() {
                                    Err(e) => trace_error_exit(&e, 1),
                                    Ok(_) => info!("Ok"),
                                }
                            })
                    },
                    Action::Reset => {
                        Counter::load(String::from(name), rt.store())
                            .map(|mut counter| {
                                match counter.reset() {
                                    Err(e) => trace_error_exit(&e, 1),
                                    Ok(_) => info!("Ok"),
                                }
                            })
                    },
                    Action::Set => {
                        let kv = String::from(name).into_kv();
                        if kv.is_none() {
                            warn!("Not a key-value pair: '{}'", name);
                            exit(1);
                        }
                        let (key, value) = kv.unwrap().into();
                        let value = FromStr::from_str(&value[..]);
                        if value.is_err() {
                            warn!("Not a integer: '{:?}'", value);
                            exit(1);
                        }
                        let value : i64 = value.unwrap();
                        Counter::load(String::from(key), rt.store())
                            .map(|mut counter| {
                                match counter.set(value) {
                                    Err(e) => trace_error_exit(&e, 1),
                                    Ok(_) => info!("Ok"),
                                }
                            })
                    },
                }
                .map_err(|e| trace_error(&e))
                .ok();
            },
            |name| {
                debug!("Call: {}", name);
                match name {
                    "create"      => create(&rt),
                    "delete"      => delete(&rt),
                    "interactive" => interactive(&rt),
                    "list"        => list(&rt),
                    _ => {
                        debug!("Unknown command"); // More error handling
                    },
                };
            })
}

