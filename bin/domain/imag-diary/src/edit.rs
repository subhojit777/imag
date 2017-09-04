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

use std::process::exit;

use chrono::naive::NaiveDateTime;

use libimagdiary::diary::Diary;
use libimagdiary::diaryid::DiaryId;
use libimagdiary::error::DiaryErrorKind as DEK;
use libimagdiary::error::MapErrInto;
use libimagentryedit::edit::Edit;
use libimagrt::runtime::Runtime;
use libimagerror::trace::MapErrTrace;
use libimagerror::into::IntoError;
use libimagtimeui::datetime::DateTime;
use libimagtimeui::parse::Parse;
use libimagutil::warn_exit::warn_exit;
use libimagerror::trace::trace_error_exit;

use util::get_diary_name;

pub fn edit(rt: &Runtime) {
    let diaryname = get_diary_name(rt).unwrap_or_else(|| warn_exit("No diary name", 1));

    rt.cli()
        .subcommand_matches("edit")
        .unwrap()
        .value_of("datetime")
        .and_then(DateTime::parse)
        .map(|dt| dt.into())
        .map(|dt: NaiveDateTime| DiaryId::from_datetime(diaryname.clone(), dt))
        .or_else(|| {
            rt.store()
                .get_youngest_entry_id(&diaryname)
                .map(|optid| match optid {
                    Ok(id) => id,
                    Err(e) => trace_error_exit(&e, 1),
                })
        })
        .ok_or_else(|| {
            error!("No entries in diary. Aborting");
            exit(1)
        })
        .and_then(|id| rt.store().get(id))
        .map(|opte| match opte {
            Some(mut e) => e.edit_content(rt).map_err_into(DEK::IOError),
            None        => Err(DEK::EntryNotInDiary.into_error()),
        })
        .map_err_trace()
        .ok();
}


