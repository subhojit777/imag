//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

extern crate crossbeam;
extern crate clap;
#[macro_use] extern crate version;
#[macro_use] extern crate log;
extern crate walkdir;

extern crate libimagrt;
extern crate libimagerror;

use std::env;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;
use std::io::ErrorKind;

use walkdir::WalkDir;
use crossbeam::*;
use clap::{Arg, AppSettings, SubCommand};

use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error;

/// Returns the helptext, putting the Strings in cmds as possible
/// subcommands into it
fn help_text(cmds: Vec<String>) -> String {
    let text = format!(r#"

     _
    (_)_ __ ___   __ _  __ _
    | | '_ \` _ \/ _\`|/ _\`|
    | | | | | | | (_| | (_| |
    |_|_| |_| |_|\__,_|\__, |
                       |___/
    -------------------------

    Usage: imag [--version | --versions | -h | --help] <command> <args...>

    imag - the personal information management suite for the commandline

    imag is a PIM suite for the commandline. It consists of several commands,
    called "modules". Each module implements one PIM aspect and all of these
    modules can be used independently.

    Available commands:

    {imagbins}

    Call a command with 'imag <command> <args>'
    Each command can be called with "--help" to get the respective helptext.

    Please visit https://github.com/matthiasbeyer/imag to view the source code,
    follow the development of imag or maybe even contribute to imag.

    imag is free software. It is released under the terms of LGPLv2.1

    (c) 2016 Matthias Beyer and contributors"#, imagbins = cmds.into_iter()
        .map(|cmd| format!("\t{}\n", cmd))
        .fold(String::new(), |s, c| {
            let s = s + c.as_str();
            s
        }));
    text
}

/// Returns the list of imag-* executables found in $PATH
fn get_commands() -> Vec<String> {
    let path = env::var("PATH");
    if path.is_err() {
        println!("PATH error: {:?}", path);
        exit(1);
    }
    let pathelements = path.unwrap();
    let pathelements = pathelements.split(":");

    let joinhandles : Vec<ScopedJoinHandle<Vec<String>>> = pathelements
        .map(|elem| {
            crossbeam::scope(|scope| {
                scope.spawn(|| {
                    WalkDir::new(elem)
                        .max_depth(1)
                        .into_iter()
                        .filter(|path| {
                            match path {
                                &Ok(ref p) => p.file_name()
                                    .to_str()
                                    .map_or(false, |filename| filename.starts_with("imag-")),
                                &Err(_)   => false,
                            }
                        })
                        .filter_map(|x| x.ok())
                        .filter_map(|path| {
                           path.file_name()
                               .to_str()
                               .and_then(|s| s.splitn(2, "-").nth(1).map(String::from))
                        })
                        .collect()
                })
            })
        })
        .collect();

    let mut execs = vec![];
    for joinhandle in joinhandles.into_iter() {
        let mut v = joinhandle.join();
        execs.append(&mut v);
    }

    execs
}


fn main() {
    // Initialize the Runtime and build the CLI
    let appname  = "imag";
    let version  = &version!();
    let about    = "imag - the PIM suite for the commandline";
    let commands = get_commands();
    let helptext = help_text(commands.clone());
    let app      = Runtime::get_default_cli_builder(appname, version, about)
        .settings(&[AppSettings::AllowExternalSubcommands, AppSettings::ArgRequiredElseHelp])
        .arg(Arg::with_name("version")
             .long("version")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Get the version of imag"))
        .arg(Arg::with_name("versions")
             .long("versions")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Get the versions of the imag commands"))
        .subcommand(SubCommand::with_name("help").help("Show help"))
        .help(helptext.as_str());
    let rt = Runtime::new(app)
        .unwrap_or_else(|e| {
            println!("Runtime couldn't be setup. Exiting");
            trace_error(&e);
            exit(1);
        });
    let matches = rt.cli();

    debug!("matches: {:?}", matches);

    // Begin checking for arguments

    if matches.is_present("version") {
        debug!("Showing version");
        println!("imag {}", &version!()[..]);
        exit(0);
    }

    if matches.is_present("versions") {
        debug!("Showing versions");
        let mut result = vec![];
        for command in commands.iter() {
            result.push(crossbeam::scope(|scope| {
                scope.spawn(|| {
                    let v = Command::new(format!("imag-{}",command)).arg("--version").output();
                    match v {
                        Ok(v) => match String::from_utf8(v.stdout) {
                            Ok(s) => format!("{:10} -> {}", command, s),
                            Err(e) => format!("Failed calling {} -> {:?}", command, e),
                        },
                        Err(e) => format!("Failed calling {} -> {:?}", command, e),
                    }
                })
            }))
        }

        for versionstring in result.into_iter().map(|handle| handle.join()) {
            // The amount of newlines may differ depending on the subprocess
            println!("{}", versionstring.trim());
        }
    }

    // Matches any subcommand given
    match matches.subcommand() {
        (subcommand, Some(scmd)) => {
            // Get all given arguments and further subcommands to pass to
            // the imag-<> binary
            // Providing no arguments is OK, and is therefore ignored here
            let subcommand_args : Vec<&str> = match scmd.values_of("") {
                Some(values) => values.collect(),
                None => Vec::new()
            };
            
            // Typos happen, so check if the given subcommand is one found in $PATH
            if !commands.contains(&String::from(subcommand)) {
                println!("No such command: 'imag-{}'", subcommand);
                println!("See 'imag --help' for available subcommands");
                exit(2);
            }
    
            debug!("Calling 'imag-{}' with args: {:?}", subcommand, subcommand_args);

            // Create a Command, and pass it the gathered arguments
            match Command::new(format!("imag-{}", subcommand))
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .args(&subcommand_args[..])
                .spawn()
                .and_then(|mut handle| handle.wait())
            {
                Ok(exit_status) => {
                    if !exit_status.success() {
                        debug!("{} exited with non-zero exit code: {:?}", subcommand, exit_status);
                        println!("{} exited with non-zero exit code", subcommand);
                        exit(exit_status.code().unwrap_or(1));
                    }
                    debug!("Successful exit!");
                },

                Err(e) => {
                    debug!("Error calling the subcommand");
                    match e.kind() {
                        ErrorKind::NotFound => {
                            // With the check above, this absolutely should not happen.
                            // Keeping it to be safe
                            println!("No such command: 'imag-{}'", subcommand);
                            println!("See 'imag --help' for available subcommands");
                            exit(2);
                        },
                        ErrorKind::PermissionDenied => {
                            println!("No permission to execute: 'imag-{}'", subcommand);
                            exit(1);
                        },
                        _ => {
                            println!("Error spawning: {:?}", e);
                            exit(1);
                        }
                    }
                }
            }
        },
        // Calling for example 'imag --versions' will lead here, as this option does not exit.
        // There's nothing to do in such a case
        _ => {},
    }
}
