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

use libimagerror::trace::trace_error;
use libimagerror::iter::TraceIterator;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;

use libimagentrytimetrack::timetracking::TimeTracking;
use libimagentrytimetrack::tag::TimeTrackingTag;
use libimagentrytimetrack::timetrackingstore::*;
use libimagentrytimetrack::iter::get::GetTimeTrackIter;

pub fn stop(rt: &Runtime) -> i32 {
    let (_, cmd) = rt.cli().subcommand();
    let cmd = cmd.unwrap(); // checked in main()

    let stop_time = match cmd.value_of("stop-time").map(::chrono::naive::NaiveDateTime::from_str) {
        None          => ::chrono::offset::Local::now().naive_local(),
        Some(Ok(ndt)) => ndt,
        Some(Err(e))  => {
            trace_error(&e);
            error!("Cannot continue, not having stop time");
            return 1
        },
    };

    // TODO: We do not yet support stopping all tags by simply calling the "stop" subcommand!

    let tags : Vec<TimeTrackingTag> = cmd.values_of("tags")
        .unwrap() // enforced by clap
        .map(String::from)
        .map(TimeTrackingTag::from)
        .collect();

    let iter : GetTimeTrackIter = match rt.store().get_timetrackings() {
        Ok(i) => i,
        Err(e) => {
            error!("Getting timetrackings failed");
            trace_error(&e);
            return 1
        }

    };

    // Filter all timetrackings for the ones that are not yet ended.
    iter.trace_unwrap()
        .filter_map(|elem| {
            // check whether end-time is set
            let has_end_time = match elem.get_end_datetime() {
                Ok(x)  => x.is_some(),
                Err(e) => {
                    warn!("Error checking {} whether End-time is set", elem.get_location());
                    trace_error(&e);
                    false
                }
            };

            // Filter the not-yet-ended timetrackings for the ones that should be ended via
            // the tag specification
            let stopping_tag_is_present : bool = elem
                .get_timetrack_tag()
                .map(|t| tags.contains(&t))
                .unwrap_or(false);

            if (!has_end_time) && stopping_tag_is_present {
                Some(elem)
            } else {
                None
            }
        })

    // for each of these timetrackings, end them
    // for each result, print the backtrace (if any)
    .fold(0, |acc, mut elem| match elem.set_end_datetime(stop_time.clone()) {
        Err(e) => { // if there was an error
            trace_error(&e); // trace
            1 // set exit code to 1
        },
        Ok(_) => {
            debug!("Setting end time worked: {:?}", elem);

            // Keep the exit code
            acc
        }
    })
}

