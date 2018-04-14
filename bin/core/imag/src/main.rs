//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

extern crate clap;
#[macro_use] extern crate log;
extern crate walkdir;
extern crate toml;
extern crate toml_query;
#[macro_use] extern crate is_match;

#[macro_use] extern crate libimagrt;
extern crate libimagerror;

use std::env;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;
use std::io::ErrorKind;
use std::io::{stdout, Stdout, Write};
use std::collections::BTreeMap;
use std::path::PathBuf;

use walkdir::WalkDir;
use clap::{Arg, ArgMatches, AppSettings, SubCommand};
use toml::Value;
use toml_query::read::TomlValueReadExt;

use libimagrt::error::RuntimeErrorKind;
use libimagrt::runtime::Runtime;
use libimagrt::spec::CliSpec;
use libimagerror::io::ToExitCode;
use libimagerror::exit::ExitUnwrap;
use libimagerror::trace::trace_error;

/// Returns the helptext, putting the Strings in cmds as possible
/// subcommands into it
fn help_text(cmds: Vec<String>) -> String {
    format!(r#"

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

    (c) 2016 Matthias Beyer and contributors"#,
        imagbins = cmds
            .into_iter()
            .map(|cmd| format!("\t{}\n", cmd))
            .fold(String::new(), |s, c| {
                let s = s + c.as_str();
                s
            }))
}

/// Returns the list of imag-* executables found in $PATH
fn get_commands(out: &mut Stdout) -> Vec<String> {
    let mut v = match env::var("PATH") {
        Err(e) => {
            let _ = writeln!(out, "PATH error: {:?}", e)
                .to_exit_code()
                .unwrap_or_exit();
            exit(1)
        },

        Ok(path) => path
            .split(":")
            .flat_map(|elem| {
                WalkDir::new(elem)
                    .max_depth(1)
                    .into_iter()
                    .filter(|path| match *path {
                        Ok(ref p) => p.file_name().to_str().map_or(false, |f| f.starts_with("imag-")),
                        Err(_)    => false,
                    })
                    .filter_map(Result::ok)
                    .filter_map(|path| path
                        .file_name()
                       .to_str()
                       .and_then(|s| s.splitn(2, "-").nth(1).map(String::from))
                    )
            })
            .collect::<Vec<String>>()
    };

    v.sort();
    v
}


fn main() {
    // Initialize the Runtime and build the CLI
    let appname  = "imag";
    let version  = make_imag_version!();
    let about    = "imag - the PIM suite for the commandline";
    let mut out  = stdout();
    let commands = get_commands(&mut out);
    let helptext = help_text(commands.clone());
    let mut app  = Runtime::get_default_cli_builder(appname, &version, about)
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
        .after_help(helptext.as_str());

    let long_help = {
        let mut v = vec![];
        if let Err(e) = app.write_long_help(&mut v) {
            eprintln!("Error: {:?}", e);
            exit(1);
        }
        String::from_utf8(v).unwrap_or_else(|_| { eprintln!("UTF8 Error"); exit(1) })
    };
    {
        let print_help = app.clone().get_matches().subcommand_name().map(|h| h == "help").unwrap_or(false);
        if print_help {
            let _ = writeln!(out, "{}", long_help)
                .to_exit_code()
                .unwrap_or_exit();
            exit(0)
        }
    }

    let matches = app.matches();
    let rtp = ::libimagrt::runtime::get_rtp_match(&matches);
    let configpath = matches
        .value_of(Runtime::arg_config_name())
        .map_or_else(|| rtp.clone(), PathBuf::from);
    debug!("Config path = {:?}", configpath);
    let config = match ::libimagrt::configuration::fetch_config(&configpath) {
            Ok(c) => Some(c),
            Err(e) => if !is_match!(e.kind(), &RuntimeErrorKind::ConfigNoConfigFileFound) {
                trace_error(&e);
                ::std::process::exit(1)
            } else {
                println!("No config file found.");
                println!("Continuing without configuration file");
                None
            },
    };

    debug!("matches: {:?}", matches);

    // Begin checking for arguments

    if matches.is_present("version") {
        debug!("Showing version");
        let _ = writeln!(out, "imag {}", env!("CARGO_PKG_VERSION"))
            .to_exit_code()
            .unwrap_or_exit();
        exit(0);
    }

    if matches.is_present("versions") {
        debug!("Showing versions");
        commands
            .iter()
            .map(|command| {
                match Command::new(format!("imag-{}", command))
                    .stdin(::std::process::Stdio::inherit())
                    .stdout(::std::process::Stdio::inherit())
                    .stderr(::std::process::Stdio::inherit())
                    .arg("--version")
                    .output()
                    .map(|v| v.stdout)
                {
                    Ok(s) => match String::from_utf8(s) {
                        Ok(s) => format!("{:15} -> {}", command, s),
                        Err(e) => format!("UTF8 Error while working with output of imag{}: {:?}", command, e),
                    },
                    Err(e) => format!("Failed calling imag-{} -> {:?}", command, e),
                }
            })
            .fold((), |_, line| {
                // The amount of newlines may differ depending on the subprocess
                let _ = writeln!(out, "{}", line.trim())
                    .to_exit_code()
                    .unwrap_or_exit();
            });

        exit(0);
    }

    let aliases = match fetch_aliases(config.as_ref()) {
        Ok(aliases) => aliases,
        Err(e)      => {
            let _ = writeln!(out, "Error while fetching aliases from configuration file")
                .to_exit_code()
                .unwrap_or_exit();
            debug!("Error = {:?}", e);
            let _ = writeln!(out, "Aborting")
                .to_exit_code()
                .unwrap_or_exit();
            exit(1);
        }
    };

    // Matches any subcommand given
    match matches.subcommand() {
        (subcommand, Some(scmd)) => {
            // Get all given arguments and further subcommands to pass to
            // the imag-<> binary
            // Providing no arguments is OK, and is therefore ignored here
            let mut subcommand_args : Vec<String> = match scmd.values_of("") {
                Some(values) => values.map(String::from).collect(),
                None => Vec::new()
            };

            forward_commandline_arguments(&matches, &mut subcommand_args);

            let subcommand = String::from(subcommand);
            let subcommand = aliases.get(&subcommand).cloned().unwrap_or(subcommand);

            debug!("Calling 'imag-{}' with args: {:?}", subcommand, subcommand_args);

            // Create a Command, and pass it the gathered arguments
            match Command::new(format!("imag-{}", subcommand))
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .args(&subcommand_args[..])
                .spawn()
                .and_then(|mut c| c.wait())
            {
                Ok(exit_status) => {
                    if !exit_status.success() {
                        debug!("imag-{} exited with non-zero exit code: {:?}", subcommand, exit_status);
                        eprintln!("imag-{} exited with non-zero exit code", subcommand);
                        exit(exit_status.code().unwrap_or(1));
                    }
                    debug!("Successful exit!");
                },

                Err(e) => {
                    debug!("Error calling the subcommand");
                    match e.kind() {
                        ErrorKind::NotFound => {
                            let _ = writeln!(out, "No such command: 'imag-{}'", subcommand)
                                .to_exit_code()
                                .unwrap_or_exit();
                            let _ = writeln!(out, "See 'imag --help' for available subcommands")
                                .to_exit_code()
                                .unwrap_or_exit();
                            exit(1);
                        },
                        ErrorKind::PermissionDenied => {
                            let _ = writeln!(out, "No permission to execute: 'imag-{}'", subcommand)
                                .to_exit_code()
                                .unwrap_or_exit();
                            exit(1);
                        },
                        _ => {
                            let _ = writeln!(out, "Error spawning: {:?}", e)
                                .to_exit_code()
                                .unwrap_or_exit();
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

fn fetch_aliases(config: Option<&Value>) -> Result<BTreeMap<String, String>, String> {
    let cfg   = config.ok_or_else(|| String::from("No configuration found"))?;
    let value = cfg
        .read("imag.aliases")
        .map_err(|_| String::from("Reading from config failed"));

    match value? {
        None                         => Ok(BTreeMap::new()),
        Some(&Value::Table(ref tbl)) => {
            let mut alias_mappings = BTreeMap::new();

            for (k, v) in tbl {
                match v {
                    &Value::String(ref alias)      => {
                        alias_mappings.insert(alias.clone(), k.clone());
                    },
                    &Value::Array(ref aliases) => {
                        for alias in aliases {
                            match alias {
                                &Value::String(ref s) => {
                                    alias_mappings.insert(s.clone(), k.clone());
                                },
                                _ => {
                                    let e = format!("Not all values are a String in 'imag.aliases.{}'", k);
                                    return Err(e);
                                }
                            }
                        }
                    },

                    _ => {
                        let msg = format!("Type Error: 'imag.aliases.{}' is not a table or string", k);
                        return Err(msg);
                    },
                }
            }

            Ok(alias_mappings)
        },

        Some(_) => Err(String::from("Type Error: 'imag.aliases' is not a table")),
    }
}

fn forward_commandline_arguments(m: &ArgMatches, scmd: &mut Vec<String>) {
    let push = |flag: Option<&str>, val_name: &str, m: &ArgMatches, v: &mut Vec<String>| {
        let _ = m
            .value_of(val_name)
            .map(|val| {
                let flag = format!("--{}", flag.unwrap_or(val_name));
                v.insert(0, String::from(val));
                v.insert(0, flag);
            });
    };

    push(Some("verbose"),
         Runtime::arg_verbosity_name(), m , scmd);

    push(Some("debug"),
         Runtime::arg_debugging_name(), m , scmd);

    push(Some("no-color"),
         Runtime::arg_no_color_output_name(), m , scmd);

    push(Some("config"),
         Runtime::arg_config_name(), m , scmd);

    push(Some("override-config"),
         Runtime::arg_config_override_name(), m , scmd);

    push(Some("rtp"),
         Runtime::arg_runtimepath_name(), m , scmd);

    push(Some("store"),
         Runtime::arg_storepath_name(), m , scmd);

    push(Some("editor"),
         Runtime::arg_editor_name(), m , scmd);

    push(None , Runtime::arg_logdest_name()                         , m , scmd);

}

