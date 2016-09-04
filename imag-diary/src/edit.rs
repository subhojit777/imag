use std::process::exit;
use chrono::naive::datetime::NaiveDateTime;

use libimagdiary::diary::Diary;
use libimagdiary::diaryid::DiaryId;
use libimagdiary::error::DiaryErrorKind as DEK;
use libimagdiary::error::MapErrInto;
use libimagentryedit::edit::Edit;
use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error;
use libimagerror::into::IntoError;
use libimagtimeui::datetime::DateTime;
use libimagtimeui::parse::Parse;

use util::get_diary_name;

pub fn edit(rt: &Runtime) {
    let diaryname = get_diary_name(rt);
    if diaryname.is_none() {
        warn!("No diary name");
        exit(1);
    }
    let diaryname = diaryname.unwrap();
    let diary = Diary::open(rt.store(), &diaryname[..]);

    let datetime : Option<NaiveDateTime> = rt
        .cli()
        .subcommand_matches("edit")
        .unwrap()
        .value_of("datetime")
        .and_then(DateTime::parse)
        .map(|dt| dt.into());

    let to_edit = match datetime {
        Some(dt) => Some(diary.retrieve(DiaryId::from_datetime(diaryname.clone(), dt))),
        None     => diary.get_youngest_entry(),
    };

    match to_edit {
        Some(Ok(mut e)) => e.edit_content(rt).map_err_into(DEK::IOError),

        Some(Err(e)) => Err(e),
        None => Err(DEK::EntryNotInDiary.into_error()),
    }
    .map_err(|e| trace_error(&e)).ok();
}


