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

use clap::ArgMatches;
use chrono::naive::NaiveDate;
use chrono::naive::NaiveDateTime;

use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error;
use libimagtimetrack::error::TimeTrackError as TTE;
use libimagtimetrack::tag::TimeTrackingTag;
use libimagtimetrack::timetrackingstore::TimeTrackStore;
use libimagerror::trace::MapErrTrace;

const DATE_TIME_PARSE_FMT : &'static str    = "%Y-%m-%dT%H:%M:%S";
const DATE_PARSE_FMT : &'static str         = "%Y-%m-%d";

pub fn track(rt: &Runtime) -> i32 {
    let (_, cmd) = rt.cli().subcommand();
    let cmd = cmd.unwrap(); // checked in main()

    // Gets the appropriate time from the commandline or None on error (errors already logged, so
    // callee can directly return in case of error
    fn get_time(cmd: &ArgMatches, clap_name: &str, errname: &str) -> Option<NaiveDateTime> {
        match cmd.value_of(clap_name) {
            Some("now") => Some(::chrono::offset::Local::now().naive_local()),
            Some(els) => {
                match NaiveDateTime::parse_from_str(els, DATE_TIME_PARSE_FMT).map_err(TTE::from) {
                    Ok(ndt) => Some(ndt),
                    Err(e_ndt) => {
                        match NaiveDate::parse_from_str(els, DATE_PARSE_FMT).map_err(TTE::from) {
                            Ok(ndt) => Some(ndt.and_hms(0, 0, 0)),
                            Err(e_nd) => {
                                error!("Cannot parse date {}:", errname);
                                trace_error(&e_nd);

                                error!("Cannot parse date-time {}:", errname);
                                trace_error(&e_ndt);
                                exit(1)
                            }
                        }
                    }
                }
            }
            None => {
                error!("Not specified in commandline: {}", clap_name);
                exit(1)
            }
        }
    }

    let start = match get_time(&cmd, "start-time", "start") {
        Some(t) => t,
        None    => return 1,
    };

    let stop = match get_time(&cmd, "end-time", "stop") {
        Some(t) => t,
        None    => return 1,
    };

    cmd.values_of("tags")
        .unwrap() // enforced by clap
        .map(String::from)
        .map(TimeTrackingTag::from)
        .fold(0, |acc, ttt| {
            rt.store()
              .create_timetracking(&start, &stop, &ttt)
              .map_err_trace()
              .map(|_| acc)
              .unwrap_or(1)
        })
}

