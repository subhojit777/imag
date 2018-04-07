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

use std::io::Write;
use std::ops::Deref;
use std::process::exit;

use libimagcalendar::event::Event;
use libimagentryref::reference::Ref;
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagerror::trace::MapErrTrace;
use libimagrt::runtime::Runtime;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;

use prettytable::Table;
use regex::Regex;
use chrono::NaiveDateTime;
use filters::filter::Filter;

pub struct PastFilter(bool, NaiveDateTime);

impl PastFilter {
    pub fn new(show_past: bool, today: NaiveDateTime) -> Self {
        PastFilter(show_past, today)
    }
}

impl<'a> Filter<FileLockEntry<'a>> for PastFilter {

    fn filter(&self, f: &FileLockEntry) -> bool {
        self.0 || f.get_end().map_err_trace_exit_unwrap(1) >= self.1
    }
}

pub struct GrepFilter(Regex);

impl GrepFilter {
    pub fn new(grep: Regex) -> Self {
        GrepFilter(grep)
    }
}

impl<'a> Filter<FileLockEntry<'a>> for GrepFilter {

    fn filter(&self, f: &FileLockEntry) -> bool {
        use libimagutil::date::datetime_to_string;

        if self.0.is_match(&f.get_start().map(|dt| datetime_to_string(&dt)).map_err_trace_exit_unwrap(1)) {
            return true
        }

        if self.0.is_match(&f.get_end().map(|dt| datetime_to_string(&dt)).map_err_trace_exit_unwrap(1)) {
            return true
        }

        if Event::get_location(f.deref())
            .map_err_trace_exit_unwrap(1)
            .map(|s| self.0.is_match(&s))
            .unwrap_or(false)
        {
            return true
        }

        if f.get_categories().map_err_trace_exit_unwrap(1).iter().any(|c| self.0.is_match(&c)) {
            return true
        }

        if f.get_description().map_err_trace_exit_unwrap(1).map(|s| self.0.is_match(&s)).unwrap_or(false) {
            return true
        }

        if f.get_uid().map_err_trace_exit_unwrap(1).map(|s| self.0.is_match(&s)).unwrap_or(false) {
            return true
        }

        false
    }
}

pub fn list_events<'a, I>(rt: &Runtime, table: bool, iter: I)
    where I: Iterator<Item = FileLockEntry<'a>>
{
    let out           = rt.stdout();
    let mut outlock   = out.lock();
    let get_list_data = |event: &FileLockEntry| {
            let start = event
                .get_start()
                .map(|dt| ::libimagutil::date::datetime_to_string(&dt))
                .map_err_trace_exit_unwrap(1);

            let end = event
                .get_end()
                .map(|dt| ::libimagutil::date::datetime_to_string(&dt))
                .map_err_trace_exit_unwrap(1);

            let desc = event
                .get_description()
                .map_err_trace_exit_unwrap(1)
                .unwrap_or_else(|| String::from("<no description>"));

            (start, end, desc)
    };

    if table {
        let mut tab = Table::new();
        tab.add_row(row!["Start", "End", "Description"]);

        iter.for_each(|event| {
            let (start, end, desc) = get_list_data(&event);
            tab.add_row(row![start, end, desc]);
        });

        let _ = tab.print(&mut out.lock())
            .unwrap_or_else(|e| {
                error!("IO error: {:?}", e);
                exit(1)
            });
    } else {
        iter.for_each(|event| {
            let (start, end, desc) = get_list_data(&event);
            let hash               = event.get_hash().map_err_trace_exit_unwrap(1);

            let _ = writeln!(outlock, "{}: {} - {} - {}", hash, start, end, desc)
                .to_exit_code()
                .unwrap_or_exit();
        });
    }
}

pub fn show_events<'a, I>(rt: &Runtime, iter: I)
    where I: Iterator<Item = FileLockEntry<'a>>
{
    let out           = rt.stdout();
    let mut outlock   = out.lock();
    let get_show_data = |event: &FileLockEntry| {
            let start = event
                .get_start()
                .map(|dt| ::libimagutil::date::datetime_to_string(&dt))
                .map_err_trace_exit_unwrap(1);

            let end = event
                .get_end()
                .map(|dt| ::libimagutil::date::datetime_to_string(&dt))
                .map_err_trace_exit_unwrap(1);

            let desc = event
                .get_description()
                .map_err_trace_exit_unwrap(1)
                .unwrap_or_else(|| String::from("<no description>"));

            let cats = event
                .get_categories()
                .map_err_trace_exit_unwrap(1);

            let loca = Event::get_location(event.deref())
                .map_err_trace_exit_unwrap(1)
                .unwrap_or_else(|| String::from("<no location>"));

            (start, end, desc, cats, loca)
    };

    iter.for_each(|event| {
        let (s, e, d, c, l) = get_show_data(&event);
        let c               = c.join(", "); // join categories by ", "
        let hash            = event.get_hash().map_err_trace_exit_unwrap(1);

        let _ = writeln!(outlock,
                         r#"Event Id   : {hash}
                            Start      : {start}
                            End        : {end}
                            Description: {description}
                            Categories : {categories}
                            Location   : {location}
                            "#,
                            hash        = hash,
                            start       = s,
                            end         = e,
                            description = d,
                            categories  = c,
                            location    = l)
            .to_exit_code()
            .unwrap_or_exit();
    });
}

pub fn all_events<'a>(store: &'a Store) -> Box<Iterator<Item = FileLockEntry<'a>> + 'a> {
    use libimagcalendar::calendar::Calendar;
    use libimagcalendar::collection::Collection;
    use libimagcalendar::store::CalendarDataStore;
    use libimagerror::iter::TraceIterator;
    use libimagstore::iter::get::StoreIdGetIteratorExtension;
    use itertools::Itertools;

    let i = store
        .calendar_collections()
        .map_err_trace_exit_unwrap(1)
        .into_get_iter(store)
        .trace_unwrap_exit(1)
        .filter_map(|x| x)
        .map(|c| c.calendars().map_err_trace_exit_unwrap(1))
        .flatten()
        .into_get_iter(store)
        .trace_unwrap_exit(1)
        .filter_map(|x| x)
        .map(move |mut c| c.events(store))
        .trace_unwrap_exit(1)
        .flatten();

    Box::new(i)
}

pub fn today() -> NaiveDateTime {
    ::chrono::offset::Local::today()
    .and_hms_opt(0, 0, 0)
    .unwrap_or_else(|| {
        error!("BUG, please report");
        ::std::process::exit(1)
    })
    .naive_local()
}

