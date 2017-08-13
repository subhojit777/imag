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

use std::cmp::Ord;
use std::cmp::Ordering;
use std::str::FromStr;

use filters::ops::not::Not;
use filters::filter::Filter;
use itertools::Itertools;
use itertools::MinMaxResult;
use chrono::NaiveDateTime;

use libimagerror::trace::trace_error;
use libimagerror::trace::MapErrTrace;
use libimagerror::iter::TraceIterator;
use libimagstore::store::FileLockEntry;
use libimagentrytimetrack::timetrackingstore::TimeTrackStore;
use libimagentrytimetrack::timetracking::TimeTracking;
use libimagentrytimetrack::tag::TimeTrackingTag;
use libimagentrytimetrack::iter::filter::*;

use libimagrt::runtime::Runtime;

pub fn month(rt: &Runtime) -> i32 {
    let cmd = rt.cli().subcommand().1.unwrap(); // checked in main

    let filter = {
        use chrono::offset::Local;
        use chrono::naive::NaiveDate;
        use chrono::Weekday;
        use chrono::Datelike;

        let now = Local::now();

        let start = match cmd.value_of("start").map(::chrono::naive::NaiveDateTime::from_str) {
            None => NaiveDate::from_ymd(now.year(), now.month(), 1).and_hms(0, 0, 0),
            Some(Ok(dt)) => dt,
            Some(Err(e)) => {
                trace_error(&e);
                return 1
            }
        };

        let end = match cmd.value_of("end").map(::chrono::naive::NaiveDateTime::from_str) {
            None => {

                // Is it much harder to go to the last second of the current month than to the first
                // second of the next month, right?
                let (year, month)  = if now.month() == 12 {
                    (now.year() + 1, 1)
                } else {
                    (now.year(), now.month())
                };

                NaiveDate::from_ymd(year, month, 1).and_hms(0, 0, 0)
            },
            Some(Ok(dt)) => dt,
            Some(Err(e)) => {
                trace_error(&e);
                return 1
            }
        };

        let tags = cmd
            .values_of("tags")
            .map(|ts| ts.into_iter().map(String::from).map(TimeTrackingTag::from).collect());

        let start_time_filter = has_start_time_where(move |dt: &NaiveDateTime| {
            start <= *dt
        });

        let end_time_filter = has_end_time_where(move |dt: &NaiveDateTime| {
            end >= *dt
        });

        let tags_filter = move |fle: &FileLockEntry| {
            match tags {
                Some(ref tags) => has_one_of_tags(&tags).filter(fle),
                None => true,
            }
        };

        tags_filter.and(start_time_filter).and(end_time_filter)
    };

    rt.store()
        .get_timetrackings()
        .and_then(|iter| {
            iter.trace_unwrap()
                .filter(|e| filter.filter(e))
                .fold(Ok(()), |acc, e| {
                    acc.and_then(|_| {
                        debug!("Processing {:?}", e.get_location());

                        let tag   = try!(e.get_timetrack_tag());
                        debug!(" -> tag = {:?}", tag);

                        let start = try!(e.get_start_datetime());
                        debug!(" -> start = {:?}", start);

                        let end   = try!(e.get_end_datetime());
                        debug!(" -> end = {:?}", end);

                        match (start, end) {
                            (None, _)          => println!("{} has no start time.", tag),
                            (Some(s), None)    => println!("{} | {} - ...", tag, s),
                            (Some(s), Some(e)) => println!("{} | {} - {}", tag, s, e),
                        }

                        Ok(())
                    })
                })
        })
        .map(|_| 0)
        .map_err_trace()
        .unwrap_or(1)
}

