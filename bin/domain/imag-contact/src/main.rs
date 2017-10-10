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
#[macro_use] extern crate version;
extern crate toml;
extern crate toml_query;
extern crate handlebars;
extern crate vobject;
extern crate walkdir;

extern crate libimagcontact;
extern crate libimagstore;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;
extern crate libimaginteraction;
extern crate libimagentryref;

use std::process::exit;
use std::collections::BTreeMap;
use std::path::PathBuf;

use handlebars::Handlebars;
use clap::ArgMatches;
use vobject::vcard::Vcard;
use toml_query::read::TomlValueReadExt;
use toml::Value;
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

use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-contact",
                                    &version!()[..],
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
        .map_err_trace_exit(1)
        .unwrap() // safed by above call
        .into_get_iter(rt.store())
        .map(|fle| {
             let fle = fle
                .map_err_trace_exit(1)
                .unwrap()
                .ok_or_else(|| CE::from("StoreId not found".to_owned()))
                .map_err_trace_exit(1)
                .unwrap();

            fle
                .get_contact_data()
                .map(|cd| (fle, cd))
                .map(|(fle, cd)| (fle, cd.into_inner()))
                .map(|(fle, cd)| (fle, Vcard::from_component(cd)))
                .map_err_trace_exit(1)
                .unwrap()
        })
        .enumerate()
        .map(|(i, (fle, vcard))| {
            let hash = fle.get_path_hash().map_err_trace_exit(1).unwrap();
            let vcard = vcard.unwrap_or_else(|e| {
                error!("Element is not a VCARD object: {:?}", e);
                exit(1)
            });

            let data = build_data_object_for_handlebars(i, hash, &vcard);

            let s = list_format.render("format", &data)
                .map_err_trace_exit(1)
                .unwrap();
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
            .map_err_trace_exit(1)
            .unwrap();
    } else if path.is_dir() {
        for entry in WalkDir::new(path).min_depth(1).into_iter() {
            let entry = entry.map_err_trace_exit(1).unwrap();
            if entry.file_type().is_file() {
                let pb = PathBuf::from(entry.path());
                let _ = rt
                    .store()
                    .create_from_path(&pb)
                    .map_err_trace_exit(1)
                    .unwrap();
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
        .map_err_trace_exit(1)
        .unwrap()
        .ok_or(CE::from(format!("No entry for hash {}", hash)))
        .map_err_trace_exit(1)
        .unwrap()
        .get_contact_data()
        .map_err_trace_exit(1)
        .unwrap()
        .into_inner();
    let vcard = Vcard::from_component(contact_data)
        .unwrap_or_else(|e| {
            error!("Element is not a VCARD object: {:?}", e);
            exit(1)
        });

    let show_format = get_contact_print_format("contact.show_format", rt, &scmd);
    let data = build_data_object_for_handlebars(0, hash, &vcard);

    let s = show_format.render("format", &data)
        .map_err_trace_exit(1)
        .unwrap();
    println!("{}", s);
    info!("Ok");
}

fn build_data_object_for_handlebars<'a>(i: usize, hash: String, vcard: &Vcard) -> BTreeMap<&'static str, String> {
    let mut data = BTreeMap::new();
    {
        data.insert("i"            , format!("{}", i));

        /// The hash (as in libimagentryref) of the contact
        data.insert("id"           , hash);

        data.insert("ADR"          , vcard.adr()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("ANNIVERSARY"  , vcard.anniversary()
                .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("BDAY"         , vcard.bday()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("CATEGORIES"   , vcard.categories()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("CLIENTPIDMAP" , vcard.clientpidmap()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("EMAIL"        , vcard.email()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("FN"           , vcard.fullname()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("GENDER"       , vcard.gender()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("GEO"          , vcard.geo()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("IMPP"         , vcard.impp()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("KEY"          , vcard.key()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("LANG"         , vcard.lang()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("LOGO"         , vcard.logo()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("MEMBER"       , vcard.member()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("N"            , vcard.name()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("NICKNAME"     , vcard.nickname()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("NOTE"         , vcard.note()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("ORG"          , vcard.org()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("PHOTO"        , vcard.photo()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("PRIOD"        , vcard.proid()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("RELATED"      , vcard.related()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("REV"          , vcard.rev()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("ROLE"         , vcard.role()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("SOUND"        , vcard.sound()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("TEL"          , vcard.tel()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("TITLE"        , vcard.title()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("TZ"           , vcard.tz()
                    .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("UID"          , vcard.uid()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("URL"          , vcard.url()
                .into_iter().map(|c| c.raw().clone()).collect());

        data.insert("VERSION"      , vcard.version()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));
    }

    data
}

fn get_contact_print_format(config_value_path: &'static str, rt: &Runtime, scmd: &ArgMatches) -> Handlebars {
    let fmt = scmd
        .value_of("format")
        .map(String::from)
        .unwrap_or_else(|| {
            rt.config()
                .ok_or_else(|| CE::from("No configuration file".to_owned()))
                .map_err_trace_exit(1)
                .unwrap()
                .read(config_value_path)
                .map_err_trace_exit(1)
                .unwrap()
                .ok_or_else(|| CE::from("Configuration 'contact.list_format' does not exist".to_owned()))
                .and_then(|value| match *value {
                    Value::String(ref s) => Ok(s.clone()),
                    _ => Err(CE::from("Type error: Expected String at 'contact.list_format'. Have non-String".to_owned()))
                })
                .map_err_trace_exit(1)
                .unwrap()
        });

    let mut hb = Handlebars::new();
    let _ = hb
        .register_template_string("format", fmt)
        .map_err_trace_exit(1)
        .unwrap();

    hb.register_escape_fn(::handlebars::no_escape);
    ::libimaginteraction::format::register_all_color_helpers(&mut hb);
    ::libimaginteraction::format::register_all_format_helpers(&mut hb);
    hb
}

