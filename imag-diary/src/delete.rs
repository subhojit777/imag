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

use chrono::naive::datetime::NaiveDateTime;

use libimagdiary::diary::Diary;
use libimagdiary::diaryid::DiaryId;
use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error_exit;
use libimagtimeui::datetime::DateTime;
use libimagtimeui::parse::Parse;
use libimagutil::warn_exit::warn_exit;

use util::get_diary_name;

pub fn delete(rt: &Runtime) {
    use libimaginteraction::ask::ask_bool;

    let diaryname = get_diary_name(rt)
        .unwrap_or_else(|| warn_exit("No diary selected. Use either the configuration file or the commandline option", 1));

    let diary = Diary::open(rt.store(), &diaryname[..]);
    debug!("Diary opened: {:?}", diary);

    let datetime : Option<NaiveDateTime> = rt
        .cli()
        .subcommand_matches("delete")
        .unwrap()
        .value_of("datetime")
        .map(|dt| { debug!("DateTime = {:?}", dt); dt })
        .and_then(DateTime::parse)
        .map(|dt| dt.into());

    let to_del = match datetime {
        Some(dt) => Some(diary.retrieve(DiaryId::from_datetime(diaryname.clone(), dt))),
        None     => diary.get_youngest_entry(),
    };

    let to_del = match to_del {
        Some(Ok(e)) => e,

        Some(Err(e)) => trace_error_exit(&e, 1),
        None => warn_exit("No entry", 1)
    };

    if !ask_bool(&format!("Deleting {:?}", to_del.get_location())[..], Some(true)) {
        info!("Aborting delete action");
        return;
    }

    if let Err(e) = diary.delete_entry(to_del) {
        trace_error_exit(&e, 1)
    }

    info!("Ok!");
}

