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

use std::str::FromStr;

use filters::filter::Filter;

use libimagerror::trace::trace_error;
use libimagerror::trace::MapErrTrace;
use libimagerror::iter::TraceIterator;
use libimagstore::store::FileLockEntry;
use libimagtimetrack::timetrackingstore::TimeTrackStore;
use libimagtimetrack::timetracking::TimeTracking;

use libimagrt::runtime::Runtime;

pub fn list(rt: &Runtime) -> i32 {
    let (_, cmd) = rt.cli().subcommand();
    let cmd = cmd.unwrap(); // checked in main()

    let start = match cmd.value_of("start-time").map(::chrono::naive::NaiveDateTime::from_str) {
        None         => None,
        Some(Ok(dt)) => Some(dt),
        Some(Err(e)) => {
            trace_error(&e);
            None
        }
    };
    let end = match cmd.value_of("end-time").map(::chrono::naive::NaiveDateTime::from_str) {
        None         => None,
        Some(Ok(dt)) => Some(dt),
        Some(Err(e)) => {
            trace_error(&e);
            None
        }
    };

    let list_not_ended = cmd.is_present("list-not-ended");

    let start_time_filter = |timetracking: &FileLockEntry| {
        start.map(|s| match timetracking.get_start_datetime() {
            Ok(Some(dt)) => dt >= s,
            Ok(None)     => {
                warn!("Funny things are happening: Timetracking has no start time");
                false
            }
            Err(e) => {
                trace_error(&e);
                false
            }
        })
        .unwrap_or(true)
    };

    let end_time_filter = |timetracking: &FileLockEntry| {
        end.map(|s| match timetracking.get_end_datetime() {
            Ok(Some(dt)) => dt <= s,
            Ok(None)     => list_not_ended,
            Err(e)       => {
                trace_error(&e);
                false
            }
        })
        .unwrap_or(true)
    };

    let filter = start_time_filter.and(end_time_filter);

    rt.store()
        .get_timetrackings()
        .and_then(|iter| {
            iter.trace_unwrap()
                .filter(|e| filter.filter(e))
                .fold(Ok(()), |acc, e| {
                    acc.and_then(|_| {
                        debug!("Processing {:?}", e.get_location());

                        let tag   = e.get_timetrack_tag()?;
                        debug!(" -> tag = {:?}", tag);

                        let start = e.get_start_datetime()?;
                        debug!(" -> start = {:?}", start);

                        let end   = e.get_end_datetime()?;
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

