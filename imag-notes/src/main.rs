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

extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
#[macro_use] extern crate version;
extern crate itertools;

extern crate libimagnotes;
extern crate libimagrt;
extern crate libimagentryedit;
extern crate libimagentrytag;
extern crate libimagerror;
extern crate libimagutil;

use std::process::exit;

use itertools::Itertools;

use libimagentryedit::edit::Edit;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagnotes::note::Note;
use libimagerror::trace::{MapErrTrace, trace_error};
use libimagutil::info_result::*;
use libimagutil::warn_result::WarnResult;

mod ui;
use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-notes",
                                    &version!()[..],
                                    "Note taking helper",
                                    build_ui);

    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call: {}", name);
            match name {
                "create" => create(&rt),
                "delete" => delete(&rt),
                "edit"   => edit(&rt),
                "list"   => list(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            };
        });
}

fn name_from_cli(rt: &Runtime, subcmd: &str) -> String {
    rt.cli().subcommand_matches(subcmd).unwrap().value_of("name").map(String::from).unwrap()
}

fn create(rt: &Runtime) {
    let name = name_from_cli(rt, "create");
    Note::new(rt.store(), name.clone(), String::new()).map_err_trace().ok();

    if rt.cli().subcommand_matches("create").unwrap().is_present("edit") &&
            !edit_entry(rt, name) {
        exit(1);
    }
}

fn delete(rt: &Runtime) {
    Note::delete(rt.store(), String::from(name_from_cli(rt, "delete")))
        .map_err_trace()
        .map_info_str("Ok")
        .ok();
}

fn edit(rt: &Runtime) {
    edit_entry(rt, name_from_cli(rt, "edit"));
}

fn edit_entry(rt: &Runtime, name: String) -> bool {
    let mut note = match Note::get(rt.store(), name) {
        Ok(Some(note)) => note,
        Ok(None) => {
            warn!("Cannot edit nonexistent Note");
            return false
        },
        Err(e) => {
            trace_error(&e);
            warn!("Cannot edit nonexistent Note");
            return false
        },
    };

    note.edit_content(rt).map_err_trace().map_warn_err_str("Editing failed").is_ok()
}

fn list(rt: &Runtime) {
    use std::cmp::Ordering;

    Note::all_notes(rt.store())
        .map_err_trace_exit(1)
        .map(|iter| {
            let notes = iter.filter_map(|note| note.map_err_trace().ok())
                .sort_by(|note_a, note_b| {
                    if let (Ok(a), Ok(b)) = (note_a.get_name(), note_b.get_name()) {
                        return a.cmp(&b)
                    } else {
                        return Ordering::Greater;
                    }
                });

            for note in notes.iter() {
                note.get_name()
                    .map(|name| println!("{}", name))
                    .map_err_trace()
                    .ok();
            }
        })
        .ok();
}

