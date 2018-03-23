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
extern crate kairos;
extern crate chrono;
extern crate prettytable;

extern crate libimaghabit;
extern crate libimagstore;
#[macro_use] extern crate libimagrt;
extern crate libimagerror;
extern crate libimagutil;
extern crate libimaginteraction;

use std::io::Write;
use std::process::exit;

use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::{MapErrTrace, trace_error};
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimaghabit::store::HabitStore;
use libimaghabit::habit::builder::HabitBuilder;
use libimaghabit::habit::HabitTemplate;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimaginteraction::ask::ask_bool;

mod ui;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-habit",
                                    &version,
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
                "today"  => today(&rt, false),
                "status" => today(&rt, true),
                "show"   => show(&rt),
                "done"   => done(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                    exit(1)
                },
            }
        })
        .unwrap_or_else(|| today(&rt, true));
}

fn create(rt: &Runtime) {
    use kairos::parser::parse as kairos_parse;
    use kairos::parser::Parsed;
    let scmd  = rt.cli().subcommand_matches("create").unwrap();                      // safe by call from main()
    let name  = scmd.value_of("create-name").map(String::from).unwrap();             // safe by clap
    let recu  = scmd.value_of("create-date-recurr-spec").map(String::from).unwrap(); // safe by clap
    let comm  = scmd.value_of("create-comment").map(String::from).unwrap();          // safe by clap
    let date  = scmd.value_of("create-date").unwrap();                               // safe by clap

    let parsedate = |d, pname| match kairos_parse(d).map_err_trace_exit_unwrap(1) {
        Parsed::TimeType(tt) => match tt.calculate() {
            Ok(tt) => match tt.get_moment() {
                Some(mom) => mom.date(),
                None => {
                    debug!("TimeType yielded: '{:?}'", tt);
                    error!("Error: '{}' parameter does not yield a point in time", pname);
                    exit(1);
                },
            },
            Err(e) => {
                error!("Error: '{:?}'", e);
                exit(1);
            }
        },
        _ => {
            error!("Error: '{}' parameter does not yield a point in time", pname);
            exit(1);
        },
    };

    debug!("Building habit: name = {name}, basedate = {date}, recurr = {recu}, comment = {comm}",
           name = name,
           date = date,
           recu = recu,
           comm = comm);

    let hb = HabitBuilder::default()
        .with_name(name)
        .with_basedate(parsedate(date, "date"))
        .with_recurspec(recu)
        .with_comment(comm);

    let hb = if let Some(until) = scmd.value_of("create-until") {
        hb.with_until(parsedate(until, "until"))
    } else {
        hb
    };

    debug!("Builder = {:?}", hb);

    hb.build(rt.store()).map_err_trace_exit_unwrap(1);
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
//
// if `future` is false, the `rt.cli()` will be checked or a subcommand "today" and the related
// future flag. If it is true, the check will not be performed and it is assumed that `--future`
// was passed.
fn today(rt: &Runtime, future: bool) {
    use libimaghabit::error::ResultExt;
    use libimaghabit::error::HabitErrorKind as HEK;

    let (future, show_done) = {
        if !future {
            let scmd = rt.cli().subcommand_matches("today").unwrap();
            let futu = scmd.is_present("today-show-future");
            let done = scmd.is_present("today-done");
            (futu, done)
        } else {
            (true, rt.cli().subcommand_matches("status").unwrap().is_present("status-done"))
        }
    };
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
                // today or in future
                debug!("Checking {due:?} == {today:?} or (future = {fut} && {due:?} > {today:?}",
                       due = due, today = today, fut = future);
                due.map(|d| d == today || (future && d > today)).unwrap_or(false)
            })
            .collect();

        // unwrap is safe because we filtered above
        relevant.sort_by_key(|h| h.next_instance_date().map_err_trace_exit_unwrap(1).unwrap());
        relevant
    };

    let any_today_relevant = show_done || relevant
        .iter()
        .filter(|h| {
            let due = h.next_instance_date().map_err_trace_exit_unwrap(1);
            debug!("Checking: {:?} == {:?}", due, today);
            due.map(|d| d == today).unwrap_or(false) // relevant today
        })
        .count() != 0;

    debug!("Any today relevant = {}", any_today_relevant);
    if !any_today_relevant {
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

            if let Some(date) = date {
                let is_done = element
                    .instance_exists_for_date(&date)
                    .map_err_trace_exit_unwrap(1);

                if show_done || !is_done {
                    info!(" * {date}: {name}", date = date, name = name);
                }
            }
        }
    } else {
        fn lister_fn(h: &FileLockEntry) -> Vec<String> {
            debug!("Listing: {:?}", h);
            let name     = h.habit_name().map_err_trace_exit_unwrap(1);
            let basedate = h.habit_basedate().map_err_trace_exit_unwrap(1);
            let recur    = h.habit_recur_spec().map_err_trace_exit_unwrap(1);
            let due      = h.next_instance_date().map_err_trace_exit_unwrap(1)
                .map(date_to_string_helper)
                .unwrap_or_else(|| String::from("<finished>"));
            let comm     = h.habit_comment().map_err_trace_exit_unwrap(1);

            let v = vec![name, basedate, recur, due, comm];
            debug!(" -> {:?}", v);
            v
        }

        let header = ["#", "Name", "Basedate", "Recurr", "Next Due", "Comment"]
            .iter()
            .map(|s| Cell::new(s))
            .collect::<Vec<Cell>>();

        let mut table = Table::new();
        table.set_titles(Row::new(header));

        let mut empty = true;
        for (i, e) in relevant.into_iter()
            .filter(|habit| {
                show_done || {
                    habit
                        .next_instance_date()
                        .map_err_trace_exit_unwrap(1)
                        .map(|date|  {
                            habit.instance_exists_for_date(&date)
                                .map_err_trace_exit_unwrap(1)
                        })
                        .unwrap_or(false)
                }
            })
            .enumerate()
        {
            let mut v = vec![format!("{}", i)];
            let mut list = lister_fn(&e);
            v.append(&mut list);
            table.add_row(v.iter().map(|s| Cell::new(s)).collect());
            empty = false;
        }

        if !empty {
            let _ = table.print(&mut rt.stdout()).to_exit_code().unwrap_or_exit();
        }
    }
}

fn list(rt: &Runtime) {
    fn lister_fn(h: &FileLockEntry) -> Vec<String> {
        debug!("Listing: {:?}", h);
        let name     = h.habit_name().map_err_trace_exit_unwrap(1);
        let basedate = h.habit_basedate().map_err_trace_exit_unwrap(1);
        let recur    = h.habit_recur_spec().map_err_trace_exit_unwrap(1);
        let comm     = h.habit_comment().map_err_trace_exit_unwrap(1);
        let due      = h.next_instance_date().map_err_trace_exit_unwrap(1)
            .map(date_to_string_helper)
            .unwrap_or_else(|| String::from("<finished>"));

        let v = vec![name, basedate, recur, comm, due];
        debug!(" -> {:?}", v);
        v
    }

    let header = ["#", "Name", "Basedate", "Recurr", "Comment", "Next Due"]
        .iter()
        .map(|s| Cell::new(s))
        .collect::<Vec<Cell>>();

    let mut empty = true;
    let mut table = Table::new();
    table.set_titles(Row::new(header));

    let _ = rt
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
        .enumerate()
        .for_each(|(i, e)| {
            let mut v = vec![format!("{}", i)];
            let mut list = lister_fn(&e);
            v.append(&mut list);
            table.add_row(v.iter().map(|s| Cell::new(s)).collect());
            empty = false;
        });

    if !empty {
        let _ = table.print(&mut rt.stdout()).to_exit_code().unwrap_or_exit();
    }
}

fn show(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("show").unwrap();          // safe by call from main()
    let name = scmd
        .value_of("show-name")
        .map(String::from)
        .unwrap(); // safe by clap


    fn instance_lister_fn(i: &FileLockEntry) -> Vec<String> {
        use libimagutil::date::date_to_string;
        use libimaghabit::instance::HabitInstance;

        let date = date_to_string(&i.get_date().map_err_trace_exit_unwrap(1));
        let comm = i.get_comment().map_err_trace_exit_unwrap(1);

        vec![date, comm]
    }

    let header = ["#", "Date", "Comment"]
        .iter()
        .map(|s| Cell::new(s))
        .collect::<Vec<Cell>>();

    let mut table = Table::new();
    table.set_titles(Row::new(header));

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

            let _ = writeln!(rt.stdout(),
                     "{i} - {name}\nBase      : {b},\nRecurrence: {r}\nComment   : {c}\n",
                     i    = i,
                     name = name,
                     b    = basedate,
                     r    = recur,
                     c    = comm)
                .to_exit_code()
                .unwrap_or_exit();

            let mut empty = true;
            let _ = habit
                .linked_instances()
                .map_err_trace_exit_unwrap(1)
                .filter_map(|instance_id| {
                    debug!("Getting: {:?}", instance_id);
                    rt.store().get(instance_id).map_err_trace_exit_unwrap(1)
                })
                .enumerate()
                .for_each(|(i, e)| {
                    let mut v = vec![format!("{}", i)];
                    let mut instances = instance_lister_fn(&e);
                    v.append(&mut instances);
                    table.add_row(v.iter().map(|s| Cell::new(s)).collect());
                    empty = false;
                });

            if !empty {
                let _ = table.print(&mut rt.stdout()).to_exit_code().unwrap_or_exit();
            }
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
                due.map(|d| (d == today || d < today) || scmd.is_present("allow-future"))
                    .unwrap_or(false)
            })
            .filter(|h| {
                names.contains(&h.habit_name().map_err_trace_exit_unwrap(1))
            })
            .collect();

        // unwrap is safe because we filtered above
        relevant.sort_by_key(|h| h.next_instance_date().map_err_trace_exit_unwrap(1).unwrap());
        relevant
    };

    for mut r in relevant {
        let next_instance_name = r.habit_name().map_err_trace_exit_unwrap(1);
        let next_instance_date = r.next_instance_date().map_err_trace_exit_unwrap(1);
        if let Some(next) = next_instance_date {
            debug!("Creating new instance on {:?}", next);
            r.create_instance_with_date(rt.store(), &next)
                .map_err_trace_exit_unwrap(1);

            info!("Done on {date}: {name}",
                  date = libimagutil::date::date_to_string(&next),
                  name = next_instance_name);
        } else {
            info!("Ignoring: {}, because there is no due date (the habit is finised)",
                next_instance_name);
        }

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

fn date_to_string_helper(d: chrono::NaiveDate) -> String {
    libimagutil::date::date_to_string(&d)
}

