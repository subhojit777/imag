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

use std::ops::Deref;

use regex::Regex;
use filters::filter::Filter;
use itertools::Itertools;

use libimagrt::runtime::Runtime;
use libimagerror::iter::TraceIterator;
use libimagerror::trace::MapErrTrace;
use libimagcalendar::collection::Collection;
use libimagcalendar::store::CalendarDataStore;
use libimagcalendar::event::Event;
use libimagcalendar::error::Result;
use libimagcalendar::calendar::Calendar;
use libimagstore::iter::get::StoreIdGetIteratorExtension;
use libimagstore::store::FileLockEntry;

pub fn find(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("find").unwrap(); // safed by main()
    let past = scmd.is_present("find-past");
    let tabl = scmd.is_present("find-table");
    let show = scmd.is_present("find-show");
    let grep = scmd.value_of("find-grep").unwrap(); // safed by clap
    let grep = Regex::new(grep).unwrap_or_else(|e| {
        error!("Invalid regex: '{}'", grep);
        error!("{}", e);
        ::std::process::exit(1)
    });

    let today = ::chrono::offset::Local::today()
        .and_hms_opt(0, 0, 0)
        .unwrap_or_else(|| {
            error!("BUG, please report");
            ::std::process::exit(1)
        })
        .naive_local();

    let past_filter = |ev: &FileLockEntry| if past {
        ev.get_end().map_err_trace_exit_unwrap(1) >= today
    } else {
        true
    };

    let grep_filter = |ev: &FileLockEntry| grepfor(ev, &grep).map_err_trace_exit_unwrap(1);

    let filter = past_filter.and(grep_filter);

    let events = rt
        .store()
        .calendar_collections()
        .map_err_trace_exit_unwrap(1)
        .into_get_iter(rt.store())
        .trace_unwrap_exit(1)
        .filter_map(|x| x)
        .map(|c| c.calendars().map_err_trace_exit_unwrap(1))
        .flatten()
        .into_get_iter(rt.store())
        .trace_unwrap_exit(1)
        .filter_map(|x| x)
        .map(|mut c| c.events(rt.store()))
        .trace_unwrap_exit(1)
        .flatten()
        .filter(|e| filter.filter(e));

    if show {
        ::util::show_events(rt, events);
    } else {
        ::util::list_events(rt, tabl, events);
    }
}

fn grepfor<'a>(ev: &FileLockEntry<'a>, grep: &Regex) -> Result<bool> {
    use libimagutil::date::datetime_to_string;

    if grep.is_match(&ev.get_start().map(|dt| datetime_to_string(&dt))?) {
        return Ok(true)
    }

    if grep.is_match(&ev.get_end().map(|dt| datetime_to_string(&dt))?) {
        return Ok(true)
    }

    if grep.is_match(&Event::get_location(ev.deref())?) {
        return Ok(true)
    }

    if ev.get_categories()?.iter().any(|c| grep.is_match(&c)) {
        return Ok(true)
    }

    if grep.is_match(&ev.get_description()?) {
        return Ok(true)
    }

    if ev.get_uid()?.map(|s| grep.is_match(&s)).unwrap_or(false) {
        return Ok(true)
    }

    Ok(false)
}

