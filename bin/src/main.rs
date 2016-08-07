extern crate crossbeam;
extern crate clap;
#[macro_use] extern crate version;
extern crate walkdir;

extern crate libimagrt;

use std::env;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;
use std::io::ErrorKind;

use walkdir::WalkDir;
use crossbeam::*;
use clap::{Arg, SubCommand};

use libimagrt::runtime::Runtime;

fn help(cmds: Vec<String>) {
    println!(r#"

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
    "#);

    for cmd in cmds.iter() {
        println!("\t{}", cmd);
    }

    println!(r#"

    Call a command with 'imag <command> <args>'
    Each command can be called with "--help" to get the respective helptext.

    Please visit https://github.com/matthiasbeyer/imag to view the source code,
    follow the development of imag or maybe even contribute to imag.

    imag is free software. It is released under the terms of LGPLv2.1

    (c) 2016 Matthias Beyer and contributors"#);
}

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
    let appname  = "imag";
    let version  = &version!();
    let about    = "imag - the PIM suite for the commandline";
    let commands = get_commands();
    let r        = Runtime::get_default_cli_builder(appname, version, about);
    let matches  = commands
        .iter()
        .fold(r, |app, cmd| app.subcommand(SubCommand::with_name(cmd)))
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
        .arg(Arg::with_name("help")
             .long("help")
             .short("h")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Show help"))
        .get_matches();

    if matches.is_present("help") {
        help(get_commands());
        exit(0);
    }

    if matches.is_present("version") {
        println!("imag {}", &version!()[..]);
        exit(0);
    }

    if matches.is_present("versions") {
        let mut result = vec![];
        for command in commands.iter() {
            result.push(crossbeam::scope(|scope| {
                scope.spawn(|| {
                    let v = Command::new(command).arg("--version").output();
                    match v {
                        Ok(v) => match String::from_utf8(v.stdout) {
                            Ok(s) => format!("{} -> {}", command, s),
                            Err(e) => format!("Failed calling {} -> {:?}", command, e),
                        },
                        Err(e) => format!("Failed calling {} -> {:?}", command, e),
                    }
                })
            }))
        }

        for versionstring in result.into_iter().map(|handle| handle.join()) {
            println!("{}", versionstring);
        }
    }

    matches.subcommand_name()
        .map(|subcommand| {

            let mut subcommand_args = vec![];

            for arg in Runtime::arg_names() {
                matches.value_of(arg)
                    .map(|value| {
                        subcommand_args.push(arg);
                        subcommand_args.push(value);
                    });
            }

            match Command::new(format!("imag-{}", subcommand))
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .arg(subcommand)
                .args(&subcommand_args[..])
                .spawn()
                .and_then(|mut handle| handle.wait())
            {
                Ok(exit_status) => {
                    if !exit_status.success() {
                        println!("{} exited with non-zero exit code", subcommand);
                        exit(exit_status.code().unwrap_or(42));
                    }
                },

                Err(e) => {
                    match e.kind() {
                        ErrorKind::NotFound => {
                            println!("No such command: 'imag-{}'", subcommand);
                            exit(2);
                        },
                        ErrorKind::PermissionDenied => {
                            println!("No permission to execute: 'imag-{}'", subcommand);
                            exit(1);
                        },
                        _ => {
                            println!("Error spawning: {:?}", e);
                            exit(1337);
                        }
                    }
                }
            }
        });
}
