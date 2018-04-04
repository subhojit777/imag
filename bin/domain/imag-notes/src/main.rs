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

extern crate clap;
#[macro_use] extern crate log;
extern crate itertools;

extern crate libimagnotes;
#[macro_use] extern crate libimagrt;
extern crate libimagentryedit;
extern crate libimagerror;
extern crate libimagutil;
extern crate libimagstore;

use std::io::Write;
use std::process::exit;

use itertools::Itertools;

use libimagentryedit::edit::Edit;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagstore::iter::get::StoreIdGetIteratorExtension;
use libimagnotes::note::Note;
use libimagnotes::notestore::*;
use libimagerror::trace::MapErrTrace;
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagerror::iter::TraceIterator;
use libimagutil::info_result::*;
use libimagutil::warn_result::WarnResult;


mod ui;
use ui::build_ui;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-notes",
                                    &version,
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
                other    => {
                    debug!("Unknown command");
                    let _ = rt.handle_unknown_subcommand("imag-notes", other, rt.cli())
                        .map_err_trace_exit_unwrap(1)
                        .code()
                        .map(std::process::exit);
                },
            };
        });
}

fn name_from_cli(rt: &Runtime, subcmd: &str) -> String {
    rt.cli().subcommand_matches(subcmd).unwrap().value_of("name").map(String::from).unwrap()
}

fn create(rt: &Runtime) {
    let name = name_from_cli(rt, "create");
    let mut note = rt
        .store()
        .new_note(name.clone(), String::new())
        .map_err_trace_exit_unwrap(1);

    if rt.cli().subcommand_matches("create").unwrap().is_present("edit") {
        let _ = note
            .edit_content(rt)
            .map_warn_err_str("Editing failed")
            .map_err_trace_exit_unwrap(1);
    }
}

fn delete(rt: &Runtime) {
    let _ = rt.store()
        .delete_note(name_from_cli(rt, "delete"))
        .map_info_str("Ok")
        .map_err_trace_exit_unwrap(1);
}

fn edit(rt: &Runtime) {
    let name = name_from_cli(rt, "edit");
    let _ = rt
        .store()
        .get_note(name.clone())
        .map_err_trace_exit_unwrap(1)
        .map(|mut note| {
            let _ = note
                .edit_content(rt)
                .map_warn_err_str("Editing failed")
                .map_err_trace_exit_unwrap(1);
        })
        .unwrap_or_else(|| {
            error!("Cannot find note with name '{}'", name);
        });
}

fn list(rt: &Runtime) {
    use std::cmp::Ordering;

    let _ = rt
        .store()
        .all_notes()
        .map_err_trace_exit_unwrap(1)
        .into_get_iter(rt.store())
        .trace_unwrap_exit(1)
        .map(|opt| opt.unwrap_or_else(|| {
            error!("Fatal: Nonexistent entry where entry should exist");
            exit(1)
        }))
        .sorted_by(|note_a, note_b| if let (Ok(a), Ok(b)) = (note_a.get_name(), note_b.get_name()) {
            return a.cmp(&b)
        } else {
            return Ordering::Greater;
        })
        .iter()
        .for_each(|note| {
            let name = note.get_name().map_err_trace_exit_unwrap(1);
            writeln!(rt.stdout(), "{}", name)
                .to_exit_code()
                .unwrap_or_exit()
        });
}

