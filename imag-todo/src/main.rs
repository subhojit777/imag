extern crate clap;
extern crate glob;
#[macro_use] extern crate log;
extern crate serde_json;
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate task_hookrs;

extern crate libimagrt;
extern crate libimagstore;
extern crate libimagerror;
extern crate libimagtodo;

use std::process::exit;
use std::process::{Command, Stdio};
use std::io::stdin;
use std::io::BufRead;

use task_hookrs::import::{import_task, import_tasks};

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagtodo::task::IntoTask;
use libimagtodo::task::Task;
use libimagerror::trace::trace_error;

mod ui;

use ui::build_ui;
fn main() {
    let rt = generate_runtime_setup("imag-todo",
                                    &version!()[..],
                                    "Interface with taskwarrior",
                                    build_ui);

    match rt.cli().subcommand_name() {
        Some("tw-hook") => tw_hook(&rt),
        Some("list") => list(&rt),
        _ => unreachable!(),
    } // end match scmd
} // end main

fn tw_hook(rt: &Runtime) {
    let subcmd = rt.cli().subcommand_matches("tw-hook").unwrap();
    if subcmd.is_present("add") {
        let stdin     = stdin();
        let mut stdin = stdin.lock();

        match Task::import(rt.store(), stdin) {
            Ok((_, uuid)) => info!("Task {} stored in imag", uuid),
            Err(e) => {
                trace_error(&e);
                exit(1);
            }
        }
    } else if subcmd.is_present("delete") {
        // The used hook is "on-modify". This hook gives two json-objects
        // per usage und wants one (the second one) back.
        let stdin         = stdin();
        let stdin         = stdin.lock();

        match import_tasks(stdin) {
            Ok(ttasks) => for (counter, ttask) in ttasks.enumerate() {
                if counter % 2 == 1 {
                    // Only every second task is needed, the first one is the
                    // task before the change, and the second one after
                    // the change. The (maybe modified) second one is
                    // expected by taskwarrior.
                    match serde_json::ser::to_string(&ttask) {
                        Ok(val) => println!("{}", val),
                        Err(e) => {
                            trace_error(&e);
                            exit(1);
                        }
                    }

                    match ttask.status() {
                        &task_hookrs::status::TaskStatus::Deleted => {
                            match Task::delete_by_uuid(rt.store(), *ttask.uuid()) {
                                Ok(_) => println!("Deleted task {}", *ttask.uuid()),
                                Err(e) => {
                                    trace_error(&e);
                                    exit(1);
                                }
                            }
                        }
                        _ => {
                        }
                    } // end match ttask.status()
                } // end if c % 2
            },
            Err(e) => {
                trace_error(&e);
                exit(1);
            },
        }
    } else {
        // Should not be possible, as one argument is required via
        // ArgGroup
        unreachable!();
    }
}

fn list(rt: &Runtime) {
    let subcmd   = rt.cli().subcommand_matches("list").unwrap();
    let mut args = Vec::new();
    let verbose  = subcmd.is_present("verbose");
    let iter     = match Task::all(rt.store()) {
        Ok(iter) => iter,
        Err(e)   => {
            trace_error(&e);
            exit(1);
        },
    };

    for task in iter {
        match task {
            Ok(val) => {
                let uuid = match val.get_header().read("todo.uuid") {
                    Ok(Some(u)) => u,
                    Ok(None)    => continue,
                    Err(e)      => {
                        trace_error(&e);
                        continue;
                    }
                };

                if verbose {
                    args.clear();
                    args.push(format!("uuid:{} information", uuid));

                    let tw_process = Command::new("task")
                        .stdin(Stdio::null())
                        .args(&args)
                        .spawn()
                        .unwrap_or_else(|e| {
                            trace_error(&e);
                            panic!("failed");
                        });
                    let output = tw_process
                        .wait_with_output()
                        .unwrap_or_else(|e| panic!("failed to unwrap output: {}", e));
                    let outstring = String::from_utf8(output.stdout)
                        .unwrap_or_else(|e| panic!("failed to execute: {}", e));

                    println!("{}", outstring);
                } else {
                    println!("{}", match uuid {
                        toml::Value::String(s) => s,
                        _ => {
                            error!("Unexpected type for todo.uuid: {}", uuid);
                            continue;
                        },
                    });
                }
            }
            Err(e) => {
                trace_error(&e);
                continue;
            }
        } // end match task
    } // end for
}

