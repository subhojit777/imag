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
extern crate toml;
extern crate toml_query;

#[macro_use] extern crate libimagrt;
extern crate libimagerror;

use std::io::Write;
use std::io::ErrorKind;
use std::process::Command;

use toml::Value;
use toml_query::read::TomlValueReadExt;

use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagrt::setup::generate_runtime_setup;

mod ui;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-git",
                                    &version,
                                    "Helper to call git in the store",
                                    ui::build_ui);

    let execute_in_store = rt
        .config()
        .unwrap_or_else(|| {
            error!("No configuration. Please use git yourself, not via imag-git");
            error!("Won't continue without configuration.");
            ::std::process::exit(1);
        })
        .read("git.execute_in_store")
        .unwrap_or_else(|e| {
            error!("Failed to read config setting 'git.execute_in_store'");
            error!("-> {:?}", e);
            ::std::process::exit(1)
        })
        .unwrap_or_else(|| {
            error!("Missing config setting 'git.execute_in_store'");
            ::std::process::exit(1)
        });

    let execute_in_store = match *execute_in_store {
        Value::Boolean(b) => b,
        _ => {
            error!("Type error: 'git.execute_in_store' is not a boolean!");
            ::std::process::exit(1)
        }
    };

    let execpath = if execute_in_store {
        rt.store().path().to_str()
    } else {
        rt.rtp().to_str()
    }
    .map(String::from)
    .unwrap_or_else(|| {
        error!("Cannot parse to string: {:?}", rt.store().path());
        ::std::process::exit(1)
    });


    let mut command = Command::new("git");
    command
        .stdin(::std::process::Stdio::inherit())
        .stdout(::std::process::Stdio::inherit())
        .stderr(::std::process::Stdio::inherit())
        .arg("-C").arg(&execpath);

    let args = rt
        .cli()
        .values_of("")
        .map(|vs| vs.map(String::from).collect())
        .unwrap_or_else(|| vec![]);

    debug!("Adding args = {:?}", args);
    command.args(&args);

    match rt.cli().subcommand() {
        (external, Some(ext_m)) => {
            command.arg(external);
            let args = ext_m
                .values_of("")
                .map(|vs| vs.map(String::from).collect())
                .unwrap_or_else(|| vec![]);

            debug!("Adding subcommand '{}' and args = {:?}", external, args);
            command.args(&args);
        },
        _ => {},
    }

    let mut out = rt.stdout();

    debug!("Calling: {:?}", command);

    match command.spawn().and_then(|mut c| c.wait()) {
        Ok(exit_status) => {
            if !exit_status.success() {
                debug!("git exited with non-zero exit code: {:?}", exit_status);
                eprintln!("git exited with non-zero exit code");
                ::std::process::exit(exit_status.code().unwrap_or(1));
            }
            debug!("Successful exit!");
        },

        Err(e) => {
            debug!("Error calling git");
            match e.kind() {
                ErrorKind::NotFound => {
                    let _ = writeln!(out, "Cannot find 'git' executable")
                        .to_exit_code()
                        .unwrap_or_exit();
                    ::std::process::exit(1);
                },
                ErrorKind::PermissionDenied => {
                    let _ = writeln!(out, "No permission to execute: 'git'")
                        .to_exit_code()
                        .unwrap_or_exit();
                    ::std::process::exit(1);
                },
                _ => {
                    let _ = writeln!(out, "Error spawning: {:?}", e)
                        .to_exit_code()
                        .unwrap_or_exit();
                    ::std::process::exit(1);
                }
            }
        }
    }
}

