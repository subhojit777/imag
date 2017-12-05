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
extern crate kairos;

extern crate libimaghabit;
extern crate libimagstore;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;
extern crate libimagentrylist;

use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::{MapErrTrace, trace_error};
use libimaghabit::store::HabitStore;
use libimaghabit::habit::builder::HabitBuilder;
use libimaghabit::habit::HabitTemplate;
use libimagstore::store::FileLockEntry;
use libimagentrylist::listers::table::TableLister;
use libimagentrylist::lister::Lister;

mod ui;

fn main() {
    let rt = generate_runtime_setup("imag-habit",
                                    &version!()[..],
                                    "Habit tracking tool",
                                    ui::build_ui);


    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "create" => create(&rt),
                "delete" => delete(&rt),
                "list"   => list(&rt),
                "show"   => show(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                    exit(1)
                },
            }
        });
}

fn create(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("create").unwrap();                      // safe by call from main()
    let name = scmd.value_of("create-name").map(String::from).unwrap();             // safe by clap
    let recu = scmd.value_of("create-date-recurr-spec").map(String::from).unwrap(); // safe by clap
    let comm = scmd.value_of("create-comment").map(String::from).unwrap();          // safe by clap
    let date = scmd.value_of("create-date").unwrap();                               // safe by clap

    let date = match ::kairos::parser::parse(date).map_err_trace_exit_unwrap(1) {
        ::kairos::parser::Parsed::TimeType(tt) => match tt.get_moment() {
            Some(mom) => mom.date(),
            None => {
                error!("Error: 'date' parameter does not yield a point in time");
                exit(1);
            },
        },
        _ => {
            error!("Error: 'date' parameter does not yield a point in time");
            exit(1);
        },
    };

    let _ = HabitBuilder::default()
        .with_name(name)
        .with_basedate(date)
        .with_recurspec(recu)
        .with_comment(comm)
        .build(rt.store())
        .map_err_trace_exit_unwrap(1);

    info!("Ok");
}

fn delete(rt: &Runtime) {
    unimplemented!()
}

fn list(rt: &Runtime) {
    fn lister_fn(h: &FileLockEntry) -> Vec<String> {
        debug!("Listing: {:?}", h);
        let name     = h.habit_name().map_err_trace_exit_unwrap(1);
        let basedate = h.habit_basedate().map_err_trace_exit_unwrap(1);
        let recur    = h.habit_recur_spec().map_err_trace_exit_unwrap(1);
        let comm     = h.habit_comment().map_err_trace_exit_unwrap(1);
        let due      = h.next_instance_date().map_err_trace_exit_unwrap(1);
        let due      = libimaghabit::util::date_to_string(&due);

        let v = vec![name, basedate, recur, comm, due];
        debug!(" -> {:?}", v);
        v
    }

    fn lister_header() -> Vec<String> {
        ["Name", "Basedate", "Recurr", "Comment", "Next Due"].iter().map(|x| String::from(*x)).collect()
    }

    let iter = rt
        .store()
        .all_habit_templates()
        .map_err_trace_exit_unwrap(1)
        .filter_map(|id| match rt.store().get(id.clone()) {
            Ok(Some(h)) => Some(h),
            Ok(None) => {
                error!("No habit found for {:?}", id);
                None
            },
            Err(e) => {
                trace_error(&e);
                None
            },
        });


    TableLister::new(lister_fn)
        .with_header(lister_header())
        .with_idx(true)
        .list(iter)
        .map_err_trace_exit_unwrap(1);
}

fn show(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("show").unwrap();          // safe by call from main()
    let name = scmd
        .value_of("show-name")
        .map(String::from)
        .unwrap(); // safe by clap

    fn instance_lister_header() -> Vec<String> {
        ["Date", "Comment"].iter().map(|x| String::from(*x)).collect()
    }

    fn instance_lister_fn(i: &FileLockEntry) -> Vec<String> {
        use libimaghabit::util::date_to_string;
        use libimaghabit::instance::HabitInstance;

        let date = date_to_string(&i.get_date().map_err_trace_exit_unwrap(1));
        let comm = i.get_comment().map_err_trace_exit_unwrap(1);

        vec![date, comm]
    }


    let _ = rt
        .store()
        .all_habit_templates()
        .map_err_trace_exit_unwrap(1)
        .filter_map(|id| match rt.store().get(id.clone()) {
            Ok(Some(h)) => Some(h),
            Ok(None) => {
                error!("Cannot get habit for {:?} in 'show' subcommand", id);
                None
            },
            Err(e) => {
                trace_error(&e);
                None
            },
        })
        .filter(|h| h.habit_name().map(|n| name == n).map_err_trace_exit_unwrap(1))
        .enumerate()
        .map(|(i, habit)| {
            let name     = habit.habit_name().map_err_trace_exit_unwrap(1);
            let basedate = habit.habit_basedate().map_err_trace_exit_unwrap(1);
            let recur    = habit.habit_recur_spec().map_err_trace_exit_unwrap(1);
            let comm     = habit.habit_comment().map_err_trace_exit_unwrap(1);

            println!("{i} - {name}\nBase      : {b},\nRecurrence: {r}\nComment   : {c}\n",
                     i    = i,
                     name = name,
                     b    = basedate,
                     r    = recur,
                     c    = comm);

            let instances_iter = habit
                .linked_instances()
                .map_err_trace_exit_unwrap(1)
                .filter_map(|instance_id| {
                    debug!("Getting: {:?}", instance_id);
                    rt.store().get(instance_id).map_err_trace_exit_unwrap(1)
                });

            TableLister::new(instance_lister_fn)
                .with_header(instance_lister_header())
                .with_idx(true)
                .list(instances_iter)
                .map_err_trace_exit_unwrap(1);
        })
        .collect::<Vec<_>>();
}

