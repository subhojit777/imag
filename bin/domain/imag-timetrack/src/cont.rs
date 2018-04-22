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

use std::cmp::Ord;

use filters::filter::Filter;
use itertools::Itertools;
use chrono::NaiveDateTime;

use libimagerror::trace::trace_error;
use libimagerror::trace::MapErrTrace;
use libimagerror::iter::TraceIterator;
use libimagtimetrack::timetrackingstore::TimeTrackStore;
use libimagtimetrack::timetracking::TimeTracking;
use libimagtimetrack::iter::filter::*;

use libimagrt::runtime::Runtime;

pub fn cont(rt: &Runtime) -> i32 {
    let groups = rt.store()
        .get_timetrackings()
        .map_err_trace_exit_unwrap(1)
        .trace_unwrap()
        .filter(|e| has_end_time.filter(&e))
        .group_by(|elem| match elem.get_end_datetime() { // Now group them by the end time
            Ok(Some(dt)) => dt,
            Ok(None) => {
                // error. We expect all of them having an end-time.
                error!("Has no end time, but should be filtered out: {:?}", elem);
                error!("This is a bug. Please report.");
                error!("Will panic now");
                panic!("Unknown bug")
            }
            Err(e) => {
                trace_error(&e);
                NaiveDateTime::from_timestamp(0, 0) // placeholder
            }
        });

    // sort the trackings by key, so by end datetime
    let elements = {
        let mut v = vec![];
        for (key, value) in groups.into_iter() {
            v.push((key, value));
        }

        v.into_iter()
        .sorted_by(|t1, t2| {
            let (k1, _) = *t1;
            let (k2, _) = *t2;
            Ord::cmp(&k1, &k2)
        })
        .into_iter()

        // get the last one, which should be the highest one
        .last() // -> Option<_>
    };

    match elements {
        Some((_, trackings)) => {
            // and then, for all trackings
             trackings
                 .fold(Ok(0), |acc, tracking| {
                     debug!("Having tracking: {:?}", tracking);

                     acc.and_then(|_| {
                        // create a new tracking with the same tag
                         tracking
                             .get_timetrack_tag()
                             .and_then(|tag| rt.store().create_timetracking_now(&tag))
                             .map(|_| 0)
                             .map_err_trace()
                     })
                 })
        },

        None => {
            info!("No trackings to continue");
            Ok(1)
        },
    }.map_err_trace_exit_unwrap(1)
}

