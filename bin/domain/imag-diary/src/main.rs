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

#[macro_use] extern crate log;
extern crate clap;
extern crate chrono;
extern crate toml;
extern crate toml_query;
extern crate itertools;

extern crate libimagdiary;
extern crate libimagentryedit;
extern crate libimagerror;
extern crate libimaginteraction;
#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimagtimeui;
extern crate libimagutil;

use std::io::Write;

use libimagrt::setup::generate_runtime_setup;
use libimagrt::runtime::Runtime;
use libimagerror::trace::MapErrTrace;

use itertools::Itertools;

mod create;
mod delete;
mod edit;
mod list;
mod ui;
mod util;
mod view;

use create::create;
use delete::delete;
use edit::edit;
use list::list;
use view::view;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-diary",
                                    &version,
                                    "Personal Diary/Diaries",
                                    ui::build_ui);

    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "diaries" => diaries(&rt),
                "create" => create(&rt),
                "delete" => delete(&rt),
                "edit" => edit(&rt),
                "list" => list(&rt),
                "view" => view(&rt),
                other    => {
                    debug!("Unknown command");
                    let _ = rt.handle_unknown_subcommand("imag-diary", other, rt.cli())
                        .map_err_trace_exit_unwrap(1)
                        .code()
                        .map(::std::process::exit);
                },
            }
        });
}

fn diaries(rt: &Runtime) {
    use libimagdiary::diary::Diary;
    use libimagerror::io::ToExitCode;
    use libimagerror::exit::ExitUnwrap;
    use libimagerror::iter::TraceIterator;

    let out         = rt.stdout();
    let mut outlock = out.lock();

    rt.store()
        .diary_names()
        .map_err_trace_exit_unwrap(1)
        .trace_unwrap_exit(1)
        .unique()
        .for_each(|n| writeln!(outlock, "{}", n).to_exit_code().unwrap_or_exit())
}

