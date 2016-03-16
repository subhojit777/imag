#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate clap;

extern crate libimagcounter;
extern crate libimagrt;
extern crate libimagutil;

use std::process::exit;
use std::str::FromStr;

use libimagrt::runtime::Runtime;
use libimagcounter::counter::Counter;
use libimagutil::trace::trace_error;
use libimagutil::key_value_split::IntoKeyValue;

mod create;
mod delete;
mod ui;

use ui::build_ui;
use create::create;
use delete::delete;

enum Action {
    Inc,
    Dec,
    Reset,
    Set,
}

fn main() {
    let name = "imag-counter";
    let version = &version!()[..];
    let about = "Counter tool to count things";
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
                                    Err(e) => { trace_error(&e); exit(1); },
                                    Ok(_) => info!("Ok"),
                                }
                            })
                    },
                    Action::Dec => {
                        Counter::load(String::from(name), rt.store())
                            .map(|mut counter| {
                                match counter.dec() {
                                    Err(e) => { trace_error(&e); exit(1); },
                                    Ok(_) => info!("Ok"),
                                }
                            })
                    },
                    Action::Reset => {
                        Counter::load(String::from(name), rt.store())
                            .map(|mut counter| {
                                match counter.reset() {
                                    Err(e) => { trace_error(&e); exit(1); },
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
                                    Err(e) => { trace_error(&e); exit(1); },
                                    Ok(_) => info!("Ok"),
                                }
                            })
                    },
                }
                .map_err(|e| trace_error(&e));
            },
            |name| {
                debug!("Call: {}", name);
                match name {
                    "create" => create(&rt),
                    "delete" => delete(&rt),
                    _ => {
                        debug!("Unknown command"); // More error handling
                    },
                };
            })



}
