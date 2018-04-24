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

use std::str::FromStr;

use filters::filter::Filter;
use chrono::NaiveDateTime;

use libimagerror::trace::trace_error;
use libimagerror::iter::TraceIterator;
use libimagerror::trace::MapErrTrace;
use libimagrt::runtime::Runtime;

use libimagtimetrack::error::TimeTrackError as TTE;
use libimagtimetrack::timetracking::TimeTracking;
use libimagtimetrack::tag::TimeTrackingTag;
use libimagtimetrack::timetrackingstore::*;
use libimagtimetrack::iter::filter::has_end_time;
use libimagtimetrack::iter::filter::has_one_of_tags;
use libimagutil::warn_result::*;
use libimagutil::debug_result::*;

pub fn stop(rt: &Runtime) -> i32 {
    let (_, cmd) = rt.cli().subcommand();
    let cmd = cmd.unwrap(); // checked in main()

    let stop_time = match cmd.value_of("stop-time") {
        None | Some("now") => ::chrono::offset::Local::now().naive_local(),
        Some(ndt)          => match NaiveDateTime::from_str(ndt).map_err(TTE::from) {
            Ok(ndt) => ndt,
            Err(e) =>  {
                trace_error(&e);
                error!("Cannot continue, not having start time");
                return 1
            },
        }
    };

    let tags : Vec<TimeTrackingTag> = cmd.values_of("tags")
        .map(|tags| tags.map(String::from).map(TimeTrackingTag::from).collect())
        .unwrap_or_else(|| {
            // Get all timetrackings which do not have an end datetime.
            rt.store()
                .get_timetrackings()
                .map_err_trace_exit_unwrap(1)
                .trace_unwrap()
                .filter_map(|tracking| {
                    let is_none = tracking
                        .get_end_datetime()
                        .map_err_trace_exit_unwrap(1)
                        .is_none();

                    if is_none {
                        Some(tracking)
                    } else {
                        None
                    }
                })
                .map(|t| t.get_timetrack_tag())
                .map(|r| r.map_err_trace_exit_unwrap(1))
                .collect()
        });


    let filter = has_end_time.not().and(has_one_of_tags(&tags));
    rt
        .store()
        .get_timetrackings()
        .map_warn_err_str("Getting timetrackings failed")
        .map_err_trace_exit_unwrap(1)
        .trace_unwrap()

        // Filter all timetrackings for the ones that are not yet ended.
        .filter(|e| filter.filter(e))

        // for each of these timetrackings, end them
        // for each result, print the backtrace (if any)
        .fold(0, |acc, mut elem| {
            elem.set_end_datetime(stop_time.clone())
                .map_dbg(|e| format!("Setting end time worked: {:?}", e))
                .map(|_| acc)
                .map_err_trace_exit_unwrap(1)
        })
}

