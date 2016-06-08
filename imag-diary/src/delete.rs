use std::process::exit;
use chrono::naive::datetime::NaiveDateTime;

use libimagdiary::diary::Diary;
use libimagdiary::diaryid::DiaryId;
use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error;
use libimagtimeui::datetime::DateTime;
use libimagtimeui::parse::Parse;

use util::get_diary_name;

pub fn delete(rt: &Runtime) {
    use libimaginteraction::ask::ask_bool;

    let diaryname = get_diary_name(rt);
    if diaryname.is_none() {
        warn!("No diary selected. Use either the configuration file or the commandline option");
        exit(1);
    }
    let diaryname = diaryname.unwrap();

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

        Some(Err(e)) => {
            trace_error(&e);
            exit(1);
        },
        None => {
            warn!("No entry");
            exit(1);
        },
    };

    if !ask_bool(&format!("Deleting {:?}", to_del.get_location())[..], Some(true)) {
        info!("Aborting delete action");
        return;
    }

    match diary.delete_entry(to_del) {
        Ok(_) => info!("Ok"),
        Err(e) => {
            trace_error(&e);
            exit(1);
        },
    }
}

