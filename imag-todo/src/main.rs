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

use toml::Value;

use task_hookrs::import::import_tasks;
use task_hookrs::status::TaskStatus;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
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
        let stdin = stdin();
        let stdin = stdin.lock(); // implements BufRead which is required for `Task::import()`

        match Task::import(rt.store(), stdin) {
            Ok((_, line, uuid)) => info!("{}\nTask {} stored in imag", line, uuid),
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
            Ok(ttasks) => for (counter, ttask) in ttasks.iter().enumerate() {
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

                    // Taskwarrior does not have the concept of deleted tasks, but only modified
                    // ones.
                    //
                    // Here we check if the status of a task is deleted and if yes, we delete it
                    // from the store.
                    if *ttask.status() == TaskStatus::Deleted {
                        match Task::delete_by_uuid(rt.store(), *ttask.uuid()) {
                            Ok(_) => println!("Deleted task {}", *ttask.uuid()),
                            Err(e) => {
                                trace_error(&e);
                                exit(1);
                            }
                        }
                    }
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
    let subcmd  = rt.cli().subcommand_matches("list").unwrap();
    let verbose = subcmd.is_present("verbose");

    let res = Task::all(rt.store()) // get all tasks
        .map(|iter| { // and if this succeeded
            // filter out the ones were we can read the uuid
            let uuids : Vec<_> = iter.filter_map(|t| match t {
                Ok(v) => match v.get_header().read("todo.uuid") {
                    Ok(Some(Value::String(ref u))) => Some(u.clone()),
                    Ok(Some(_)) => {
                        warn!("Header type error");
                        None
                    },
                    Ok(None) => None,
                    Err(e) => {
                        trace_error(&e);
                        None
                    }
                },
                Err(e) => {
                    trace_error(&e);
                    None
                }
            })
            .collect();

            // compose a `task` call with them, ...
            let outstring = if verbose { // ... if verbose
                let output = Command::new("task")
                    .stdin(Stdio::null())
                    .args(&uuids)
                    .spawn()
                    .unwrap_or_else(|e| {
                        trace_error(&e);
                        panic!("Failed to execute `task` on the commandline. I'm dying now.");
                    })
                    .wait_with_output()
                    .unwrap_or_else(|e| panic!("failed to unwrap output: {}", e));

                String::from_utf8(output.stdout)
                    .unwrap_or_else(|e| panic!("failed to execute: {}", e))
            } else { // ... else just join them
                uuids.join("\n")
            };

            /// and then print that
            println!("{}", outstring);
        });

    if let Err(e) = res {
        trace_error(&e);
    }
}

