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

use clap::ArgMatches;

use libimagdiary::diary::Diary;
use libimagdiary::diaryid::DiaryId;
use libimagdiary::error::DiaryErrorKind as DEK;
use libimagdiary::error::ResultExt;
use libimagentryedit::edit::Edit;
use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error_exit;
use libimagerror::trace::MapErrTrace;
use libimagutil::warn_exit::warn_exit;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;

use util::get_diary_name;
use util::get_diary_timed_config;
use util::Timed;

pub fn create(rt: &Runtime) {
    let diaryname = get_diary_name(rt)
        .unwrap_or_else( || warn_exit("No diary selected. Use either the configuration file or the commandline option", 1));

    let mut entry = create_entry(rt.store(), &diaryname, rt);

    let res = if rt.cli().subcommand_matches("create").unwrap().is_present("no-edit") {
        debug!("Not editing new diary entry");
        Ok(())
    } else {
        debug!("Editing new diary entry");
        entry.edit_content(rt)
            .chain_err(|| DEK::DiaryEditError)
    };

    if let Err(e) = res {
        trace_error_exit(&e, 1);
    } else {
        info!("Ok!");
    }
}

fn create_entry<'a>(diary: &'a Store, diaryname: &str, rt: &Runtime) -> FileLockEntry<'a> {
    use util::parse_timed_string;

    let create = rt.cli().subcommand_matches("create").unwrap();

    let create_timed = create.value_of("timed")
        .map(|t| parse_timed_string(t, diaryname).map_err_trace_exit_unwrap(1))
        .map(Some)
        .unwrap_or_else(|| match get_diary_timed_config(rt, diaryname) {
            Err(e)      => trace_error_exit(&e, 1),
            Ok(Some(t)) => Some(t),
            Ok(None)    => {
                warn!("Missing config: 'diary.diaries.{}.timed'", diaryname);
                warn!("Assuming 'false'");
                None
            }
        });

    let entry = match create_timed {
        Some(timed) => {
            let id = create_id_from_clispec(&create, &diaryname, timed);
            diary.retrieve(id).chain_err(|| DEK::StoreReadError)
        },

        None => {
            debug!("Creating non-timed entry");
            diary.new_entry_today(diaryname)
        }
    };

    match entry {
        Err(e) => trace_error_exit(&e, 1),
        Ok(e) => {
            debug!("Created: {}", e.get_location());
            e
        }
    }
}


fn create_id_from_clispec(create: &ArgMatches, diaryname: &str, timed_type: Timed) -> DiaryId {
    use std::str::FromStr;

    let get_hourly_id = |create: &ArgMatches| -> DiaryId {
        let time = DiaryId::now(String::from(diaryname));
        let hr = create
            .value_of("hour")
            .map(|v| { debug!("Creating hourly entry with hour = {:?}", v); v })
            .and_then(|s| {
                FromStr::from_str(s)
                    .map_err(|_| warn!("Could not parse hour: '{}'", s))
                    .ok()
            })
            .unwrap_or(time.hour());

        time.with_hour(hr)
    };

    match timed_type {
        Timed::Hourly => {
            debug!("Creating hourly-timed entry");
            get_hourly_id(create)
        },

        Timed::Minutely => {
            let time = get_hourly_id(create);
            let min = create
                .value_of("minute")
                .map(|m| { debug!("minute = {:?}", m); m })
                .and_then(|s| {
                    FromStr::from_str(s)
                        .map_err(|_| warn!("Could not parse minute: '{}'", s))
                        .ok()
                })
                .unwrap_or(time.minute());

            time.with_minute(min)
        },
    }
}

