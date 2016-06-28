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
extern crate libimagutil;
extern crate libimagtodo;

use std::process::exit;
use std::process::{Command, Stdio};
use std::io::stdin;

use task_hookrs::import::import;

use libimagrt::runtime::Runtime;
use libimagtodo::task::IntoTask;
use libimagutil::trace::trace_error;

mod ui;

use ui::build_ui;
fn main() {

    let name = "imag-todo";
    let version = &version!()[..];
    let about = "Interface with taskwarrior";
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



    let scmd = rt.cli().subcommand_name();
    match scmd {
        Some("tw-hook") => {
            let subcmd = rt.cli().subcommand_matches("tw-hook").unwrap();
            if subcmd.is_present("add") {
                if let Ok(ttasks) = import(stdin()) {
                    for ttask in ttasks {
                        println!("{}", match serde_json::ser::to_string(&ttask) {
                            Ok(val) => val,
                            Err(e) => {
                                error!("{}", e);
                                return;
                            }
                        });
                        match ttask.into_filelockentry(rt.store()) {
                            Ok(val) => val,
                            Err(e) => {
                                trace_error(&e);
                                error!("{}", e);
                                return;
                            }
                        };
                    }
                }
            }
            else if subcmd.is_present("delete") {
                println!("To be implemented");
                //
                // Functionality to delete Entry in the store
                //
            }
            else {
                // Should not be possible, as one argument is required via
                // ArgGroup
                panic!("Reached unreachable Code");
            }
        },
        Some("exec") => {
		let subcmd = rt.cli().subcommand_matches("exec").unwrap();
		let mut args = Vec::new();
		if let Some(exec_string) = subcmd.values_of("command") {
			for e in exec_string {
				args.push(e);
			}
			let tw_process = Command::new("task").stdin(Stdio::null()).args(&args).spawn().unwrap_or_else(|e| {
				panic!("failed to execute taskwarrior: {}", e);
			});

			let output = tw_process.wait_with_output().unwrap_or_else(|e| {
				panic!("failed to unwrap output: {}", e);
			});
			let outstring = String::from_utf8(output.stdout).unwrap_or_else(|e| {
				panic!("failed to ececute: {}", e);
			});
			println!("{}", outstring);
		} else {
			panic!("faild to execute: You need to exec --command");
		}
            },
                _ => panic!("Reached unreachable Code"),
        }

}

