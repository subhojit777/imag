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

use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error;
use libimagentrytimetrack::tag::TimeTrackingTag;
use libimagentrytimetrack::timetrackingstore::TimeTrackStore;
use libimagerror::trace::MapErrTrace;

pub fn start(rt: &Runtime) -> i32 {
    let (_, cmd) = rt.cli().subcommand();
    let cmd = cmd.unwrap(); // checked in main()

    let start = match cmd.value_of("start-time") {
        None | Some("now") => ::chrono::offset::Local::now().naive_local(),
        Some(ndt)          => match ::chrono::naive::NaiveDateTime::from_str(ndt) {
            Ok(ndt) => ndt,
            Err(e) =>  {
                trace_error(&e);
                error!("Cannot continue, not having start time");
                return 1
            },
        }
    };

    cmd.values_of("tags")
        .unwrap() // enforced by clap
        .map(String::from)
        .map(TimeTrackingTag::from)
        .fold(0, |acc, ttt| {
            rt.store()
              .create_timetracking_at(&start, &ttt)
              .map_err_trace()
              .map(|_| acc)
              .unwrap_or(1)
        })
}

