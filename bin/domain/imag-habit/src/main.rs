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
extern crate chrono;

extern crate libimaghabit;
extern crate libimagstore;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;
extern crate libimagentrylist;
extern crate libimaginteraction;

use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::{MapErrTrace, trace_error};
use libimaghabit::store::HabitStore;
use libimaghabit::habit::builder::HabitBuilder;
use libimaghabit::habit::HabitTemplate;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimagentrylist::listers::table::TableLister;
use libimagentrylist::lister::Lister;
use libimaginteraction::ask::ask_bool;

mod ui;

fn main() {
    let rt = generate_runtime_setup("imag-habit",
                                    &version!()[..],
                                    "Habit tracking tool",
                                    ui::build_ui);


    let _ = rt
        .cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "create" => create(&rt),
                "delete" => delete(&rt),
                "list"   => list(&rt),
                "today"  => today(&rt),
                "show"   => show(&rt),
                "done"   => done(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                    exit(1)
                },
            }
        })
        .unwrap_or_else(|| today(&rt));
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
    use libimaghabit::instance::HabitInstance;

    let scmd = rt.cli().subcommand_matches("delete").unwrap();          // safe by call from main()
    let name = scmd.value_of("delete-name").map(String::from).unwrap(); // safe by clap
    let yes  = scmd.is_present("delete-yes");
    let delete_instances = scmd.is_present("delete-instances");

    let _ = rt
        .store()
        .all_habit_templates()
        .map_err_trace_exit_unwrap(1)
        .map(|sid| (sid.clone(), rt.store().get(sid).map_err_trace_exit_unwrap(1))) // get the FileLockEntry
        .filter(|&(_, ref habit)| match habit { // filter for name of habit == name we look for
            &Some(ref h) => h.habit_name().map_err_trace_exit_unwrap(1) == name,
            &None => false,
        })
        .filter_map(|(a, o)| o.map(|x| (a, x))) // map: (a, Option<b>) -> Option<(a, b)> -> (a, b)
        .map(|(sid, fle)| {
            if delete_instances {

                // if this does not succeed, we did something terribly wrong
                let t_name = fle.habit_name().map_err_trace_exit_unwrap(1);
                assert_eq!(t_name, name);

                let get_instance          =  |iid| rt.store().get(iid).map_err_trace_exit_unwrap(1);
                let has_template_name     =  |i: &FileLockEntry| t_name ==  i.get_template_name().map_err_trace_exit_unwrap(1);
                let instance_location     =  |i: FileLockEntry| i.get_location().clone();
                let delete_instance_by_id =  |id| {
                    let do_delete = |id| rt.store().delete(id).map_err_trace_exit_unwrap(1);
                    if !yes {
                        let q = format!("Really delete {}", id);
                        if ask_bool(&q, Some(false)) {
                            let _ = do_delete(id);
                        }
                    } else {
                        let _ = do_delete(id);
                    }
                };

                fle
                    .linked_instances()
                    .map_err_trace_exit_unwrap(1)
                    .filter_map(get_instance)
                    .filter(has_template_name)
                    .map(instance_location)
                    .map(delete_instance_by_id)
                    .collect::<Vec<_>>();
            }

            drop(fle);

            let do_delete_template = |sid| rt.store().delete(sid).map_err_trace_exit_unwrap(1);
            if !yes {
                let q = format!("Really delete template {}", sid);
                if ask_bool(&q, Some(false)) {
                    let _ = do_delete_template(sid);
                }
            } else {
                let _ = do_delete_template(sid);
            }
        })
        .collect::<Vec<_>>();

    info!("Done");
}

// Almost the same as `list()` but with other lister functions and an additional filter for only
// listing entries which are due today.
fn today(rt: &Runtime) {
    use libimaghabit::error::ResultExt;
    use libimaghabit::error::HabitErrorKind as HEK;

    fn lister_fn(h: &FileLockEntry) -> Vec<String> {
        debug!("Listing: {:?}", h);
        let name     = h.habit_name().map_err_trace_exit_unwrap(1);
        let basedate = h.habit_basedate().map_err_trace_exit_unwrap(1);
        let recur    = h.habit_recur_spec().map_err_trace_exit_unwrap(1);
        let comm     = h.habit_comment().map_err_trace_exit_unwrap(1);

        let v = vec![name, basedate, recur, comm];
        debug!(" -> {:?}", v);
        v
    }

    fn lister_header() -> Vec<String> {
        ["Name", "Basedate", "Recurr", "Comment"].iter().map(|x| String::from(*x)).collect()
    }

    let today = ::chrono::offset::Local::today().naive_local();

    let relevant : Vec<_> = { // scope, to have variable non-mutable in outer scope
        let mut relevant : Vec<_> = rt
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
            })
            .filter(|h| {
                let due = h.next_instance_date().map_err_trace_exit_unwrap(1);
                due == today || due > today // today or in future
            })
            .collect();

        relevant.sort_by_key(|h| h.next_instance_date().map_err_trace_exit_unwrap(1));
        relevant
    };

    let any_today_relevant = relevant
        .iter()
        .filter(|h| {
            let due = h.next_instance_date().map_err_trace_exit_unwrap(1);
            due == today // relevant today
        })
        .count() == 0;

    if any_today_relevant {
        let n = rt
            .cli()
            .subcommand_matches("today")
            .and_then(|am| {
                am.value_of("today-show-next-n")
                    .map(|x| {
                        x.parse::<usize>()
                            .chain_err(|| HEK::from(format!("Cannot parse String '{}' to integer", x)))
                            .map_err_trace_exit_unwrap(1)
                    })
            }).unwrap_or(5);

        info!("No Habits due today.");
        info!("Upcoming:");
        // list `n` which are relevant in the future.
        for element in relevant.iter().take(n) {
            let date = element.next_instance_date().map_err_trace_exit_unwrap(1);
            let name = element.habit_name().map_err_trace_exit_unwrap(1);

            info!(" * {date}: {name}", date = date, name = name);
        }
    } else {
        TableLister::new(lister_fn)
            .with_header(lister_header())
            .with_idx(true)
            .print_empty(false)
            .list(relevant.into_iter())
            .map_err_trace_exit_unwrap(1);
    }
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
        .print_empty(false)
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
        .filter_map(|id| get_from_store(rt.store(), id))
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
                .print_empty(false)
                .list(instances_iter)
                .map_err_trace_exit_unwrap(1);
        })
        .collect::<Vec<_>>();
}

fn done(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("done").unwrap(); // safe by call from main()
    let names : Vec<_> = scmd.values_of("done-name").unwrap().map(String::from).collect();

    let today = ::chrono::offset::Local::today().naive_local();

    let relevant : Vec<_> = { // scope, to have variable non-mutable in outer scope
        let mut relevant : Vec<_> = rt
            .store()
            .all_habit_templates()
            .map_err_trace_exit_unwrap(1)
            .filter_map(|id| get_from_store(rt.store(), id))
            .filter(|h| {
                let due = h.next_instance_date().map_err_trace_exit_unwrap(1);
                (due == today || due < today) || scmd.is_present("allow-future")
            })
            .filter(|h| {
                names.contains(&h.habit_name().map_err_trace_exit_unwrap(1))
            })
            .collect();

        relevant.sort_by_key(|h| h.next_instance_date().map_err_trace_exit_unwrap(1));
        relevant
    };

    for r in relevant.iter() {
        let next_instance_name = r.habit_name().map_err_trace_exit_unwrap(1);
        let next_instance_date = r.next_instance_date().map_err_trace_exit_unwrap(1);

        debug!("Creating new instance on {:?}", next_instance_date);
        r.create_instance_with_date(rt.store(), &next_instance_date)
            .map_err_trace_exit_unwrap(1);

        info!("Done on {date}: {name}",
              date = libimaghabit::util::date_to_string(&next_instance_date),
              name = next_instance_name);
    }
    info!("Done.");
}

/// Helper function for `Iterator::filter_map()`ing `all_habit_templates()` and `Store::get` them.
fn get_from_store<'a>(store: &'a Store, id: StoreId) -> Option<FileLockEntry<'a>> {
    match store.get(id.clone()) {
        Ok(Some(h)) => Some(h),
        Ok(None) => {
            error!("No habit found for {:?}", id);
            None
        },
        Err(e) => {
            trace_error(&e);
            None
        },
    }
}
