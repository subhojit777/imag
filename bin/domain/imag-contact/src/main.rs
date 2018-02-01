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
#[macro_use] extern crate vobject;
extern crate toml;
extern crate toml_query;
extern crate handlebars;
extern crate walkdir;
extern crate uuid;

extern crate libimagcontact;
extern crate libimagstore;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;
extern crate libimaginteraction;
extern crate libimagentryref;
extern crate libimagentryedit;

use std::process::exit;
use std::path::PathBuf;

use handlebars::Handlebars;
use clap::ArgMatches;
use vobject::vcard::Vcard;
use toml_query::read::TomlValueReadTypeExt;
use walkdir::WalkDir;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;
use libimagcontact::store::ContactStore;
use libimagcontact::error::ContactError as CE;
use libimagcontact::contact::Contact;
use libimagstore::iter::get::StoreIdGetIteratorExtension;
use libimagentryref::reference::Ref;
use libimagentryref::refstore::RefStore;

mod ui;
mod util;
mod create;

use ui::build_ui;
use util::build_data_object_for_handlebars;
use create::create;

fn main() {
    let rt = generate_runtime_setup("imag-contact",
                                    env!("CARGO_PKG_VERSION"),
                                    "Contact management tool",
                                    build_ui);


    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "list"   => list(&rt),
                "import" => import(&rt),
                "show"   => show(&rt),
                "create" => create(&rt),
                _        => {
                    error!("Unknown command"); // More error handling
                },
            }
        });
}

fn list(rt: &Runtime) {
    let scmd        = rt.cli().subcommand_matches("list").unwrap();
    let list_format = get_contact_print_format("contact.list_format", rt, &scmd);

    let _ = rt
        .store()
        .all_contacts()
        .map_err_trace_exit_unwrap(1)
        .into_get_iter(rt.store())
        .map(|fle| {
             let fle = fle
                .map_err_trace_exit_unwrap(1)
                .ok_or_else(|| CE::from("StoreId not found".to_owned()))
                .map_err_trace_exit_unwrap(1);

            fle
                .get_contact_data()
                .map(|cd| (fle, cd))
                .map(|(fle, cd)| (fle, cd.into_inner()))
                .map(|(fle, cd)| (fle, Vcard::from_component(cd)))
                .map_err_trace_exit_unwrap(1)
        })
        .enumerate()
        .map(|(i, (fle, vcard))| {
            let hash = fle.get_path_hash().map_err_trace_exit_unwrap(1);
            let vcard = vcard.unwrap_or_else(|e| {
                error!("Element is not a VCARD object: {:?}", e);
                exit(1)
            });

            let data = build_data_object_for_handlebars(i, hash, &vcard);

            let s = list_format.render("format", &data)
                .map_err_trace_exit_unwrap(1);
            println!("{}", s);
        })
        .collect::<Vec<_>>();
}

fn import(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("import").unwrap(); // secured by main
    let path = scmd.value_of("path").map(PathBuf::from).unwrap(); // secured by clap

    if !path.exists() {
        error!("Path does not exist");
        exit(1)
    }

    if path.is_file() {
        let _ = rt
            .store()
            .create_from_path(&path)
            .map_err_trace_exit_unwrap(1);
    } else if path.is_dir() {
        for entry in WalkDir::new(path).min_depth(1).into_iter() {
            let entry = entry.map_err_trace_exit_unwrap(1);
            if entry.file_type().is_file() {
                let pb = PathBuf::from(entry.path());
                let _ = rt
                    .store()
                    .create_from_path(&pb)
                    .map_err_trace_exit_unwrap(1);
                info!("Imported: {}", entry.path().to_str().unwrap_or("<non UTF-8 path>"));
            } else {
                warn!("Ignoring non-file: {}", entry.path().to_str().unwrap_or("<non UTF-8 path>"));
            }
        }
    } else {
        error!("Path is neither directory nor file");
        exit(1)
    }
}

fn show(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("show").unwrap();
    let hash = scmd.value_of("hash").map(String::from).unwrap(); // safed by clap

    let contact_data = rt.store()
        .get_by_hash(hash.clone())
        .map_err_trace_exit_unwrap(1)
        .ok_or(CE::from(format!("No entry for hash {}", hash)))
        .map_err_trace_exit_unwrap(1)
        .get_contact_data()
        .map_err_trace_exit_unwrap(1)
        .into_inner();
    let vcard = Vcard::from_component(contact_data)
        .unwrap_or_else(|e| {
            error!("Element is not a VCARD object: {:?}", e);
            exit(1)
        });

    let show_format = get_contact_print_format("contact.show_format", rt, &scmd);
    let data = build_data_object_for_handlebars(0, hash, &vcard);

    let s = show_format.render("format", &data).map_err_trace_exit_unwrap(1);
    println!("{}", s);
    info!("Ok");
}

fn get_contact_print_format(config_value_path: &'static str, rt: &Runtime, scmd: &ArgMatches) -> Handlebars {
    let fmt = scmd
        .value_of("format")
        .map(String::from)
        .unwrap_or_else(|| {
            rt.config()
                .ok_or_else(|| CE::from("No configuration file".to_owned()))
                .map_err_trace_exit_unwrap(1)
                .read_string(config_value_path)
                .map_err_trace_exit_unwrap(1)
                .ok_or_else(|| CE::from("Configuration 'contact.list_format' does not exist".to_owned()))
                .map_err_trace_exit_unwrap(1)
        });

    let mut hb = Handlebars::new();
    let _ = hb.register_template_string("format", fmt).map_err_trace_exit_unwrap(1);

    hb.register_escape_fn(::handlebars::no_escape);
    ::libimaginteraction::format::register_all_color_helpers(&mut hb);
    ::libimaginteraction::format::register_all_format_helpers(&mut hb);
    hb
}

