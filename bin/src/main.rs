extern crate crossbeam;
#[macro_use] extern crate version;
extern crate walkdir;

use std::env;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;
use std::io::ErrorKind;

use walkdir::WalkDir;
use crossbeam::*;

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
                               .map(String::from)
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

fn find_command() -> Option<String> {
    env::args().skip(1).filter(|x| !x.starts_with("-")).next()
}

fn find_flag() -> Option<String> {
    env::args().skip(1).filter(|x| x.starts_with("-")).next()
}

fn find_args(command: &str) -> Vec<String> {
    env::args()
        .skip(1)
        .position(|e| e == command)
        .map(|pos| env::args().skip(pos + 2).collect::<Vec<String>>())
        .unwrap_or(vec![])
}

fn main() {
    let commands  = get_commands();
    let mut args  = env::args();
    let _         = args.next();
    let first_arg = match find_command() {
        Some(s) => s,
        None    => match find_flag() {
            Some(s) => s,
            None => {
                help(commands);
                exit(0);
            },
        },
    };

    match &first_arg[..] {
        "--help" | "-h" => {
            help(commands);
            exit(0);
        },

        "--version"  => println!("imag {}", &version!()[..]),

        "--versions" => {
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
        },

        s => {
            match Command::new(format!("imag-{}", s))
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .args(&find_args(s)[..])
                .spawn()
                .and_then(|mut handle| handle.wait())
            {
                Ok(exit_status) => {
                    if !exit_status.success() {
                        println!("{} exited with non-zero exit code", s);
                        exit(exit_status.code().unwrap_or(42));
                    }
                },

                Err(e) => {
                    match e.kind() {
                        ErrorKind::NotFound => {
                            println!("No such command: 'imag-{}'", s);
                            exit(2);
                        },
                        ErrorKind::PermissionDenied => {
                            println!("No permission to execute: 'imag-{}'", s);
                            exit(1);
                        },
                        _ => {
                            println!("Error spawning: {:?}", e);
                            exit(1337);
                        }
                    }
                }
            }

        },
    }
}
