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

use std::process::exit;
use std::io::Write;

use chrono::NaiveDateTime;
use prettytable::Table;
use clap::ArgMatches;

use libimagcalendar::calendar::Calendar;
use libimagcalendar::error::CalendarError;
use libimagcalendar::error::Result;
use libimagcalendar::event::Event;
use libimagcalendar::store::calendars::CalendarStore;
use libimagcalendar::store::collections::CalendarCollectionStore;
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagerror::trace::MapErrTrace;
use libimagrt::runtime::Runtime;
use libimagstore::store::FileLockEntry;
use libimagutil::info_result::*;

pub fn entry(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("entry").unwrap(); // safed by main()

    match scmd.subcommand() {
        ("add", scmd)    => add(rt, scmd.unwrap()),
        ("remove", scmd) => remove(rt, scmd.unwrap()),
        ("show", scmd)   => show(rt, scmd.unwrap()),
        _ => {
            unimplemented!()
        }
    }
}

fn add<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    let collname = scmd.value_of("entry-add-collectionname").unwrap(); // safed by clap

    rt.store()
        .get_calendar_collection(&collname)
        .map_err_trace_exit_unwrap(1)
        .ok_or_else(|| format!("Collection {} does not exist", collname))
        .map_err(CalendarError::from)
        .map_err_trace_exit_unwrap(1)
        ;

    unimplemented!()
}

fn remove<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    let entryid = scmd.value_of("entry-remove-uid").map(String::from).unwrap(); // safed by clap

    let _ = rt.store().delete_calendar_by_hash(entryid).map_err_trace_exit_unwrap(1);

    unimplemented!()
}

fn show<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    let entryid = scmd.value_of("entry-show-uid").unwrap(); // safed by clap
    let use_tab = scmd.is_present("entry-show-table");

    let events = rt.store()
        .get_calendar(&entryid)
        .map_err_trace_exit_unwrap(1)
        .ok_or_else(|| format!("Entry {} does not exist", entryid))
        .map_err(CalendarError::from)
        .map_err_trace_exit_unwrap(1)
        .events(rt.store())
        .map_info(|_| format!("Showing events in {}", entryid))
        .map_err_trace_exit_unwrap(1);

    let out           = rt.stdout();
    let mut outlock   = out.lock();
    if use_tab {
        let mut tab = Table::new();
        tab.add_row(row!["Event id", "Start", "End", "Description"]);

        for event in events {
            let (id, start, end, desc) = get_event_data(&event).map_err_trace_exit_unwrap(1);
            tab.add_row(row![id, start, end, desc]);
        }

        let _ = tab.print(&mut outlock)
            .unwrap_or_else(|e| {
                error!("IO error: {:?}", e);
                exit(1)
            });
    } else {
        for event in events {
            let (id, start, end, desc) = get_event_data(&event).map_err_trace_exit_unwrap(1);
            writeln!(outlock, "{}: {} - {} - {}", id, start, end, desc)
                .to_exit_code()
                .unwrap_or_exit();
        }
    }
}

fn get_event_data(ev: &FileLockEntry) -> Result<(String, NaiveDateTime, NaiveDateTime, String)> {
    let id    = ev.get_uid()?.unwrap_or_else(|| String::from("<no id>"));
    let start = ev.get_start()?;
    let end   = ev.get_end()?;
    let desc  = ev.get_description()?;

    Ok((id, start, end, desc))
}
