#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate clap;
extern crate chrono;

extern crate libimagdiary;
extern crate libimagentrylist;
extern crate libimaginteraction;
extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;
extern crate libimagtimeui;
#[macro_use] extern crate libimagerror;

use std::path::PathBuf;
use std::process::exit;
use chrono::naive::datetime::NaiveDateTime;

use libimagdiary::diary::Diary;
use libimagdiary::diaryid::DiaryId;
use libimagdiary::error::DiaryError as DE;
use libimagdiary::error::DiaryErrorKind as DEK;
use libimagentrylist::listers::core::CoreLister;
use libimagentrylist::lister::Lister;
use libimagrt::edit::Edit;
use libimagrt::runtime::Runtime;
use libimagstore::storeid::StoreId;
use libimagerror::trace::trace_error;
use libimagtimeui::datetime::DateTime;
use libimagtimeui::parse::Parse;

mod ui;

use ui::build_ui;

fn main() {
    let name = "imag-diary";
    let version = &version!()[..];
    let about = "Personal Diary/Diaries";
    let ui = build_ui(Runtime::get_default_cli_builder(name, version, about));
    let rt = {
        let rt = Runtime::new(ui);
        if rt.is_ok() {
            rt.unwrap()
        } else {
            println!("Could not set up Runtime");
            println!("{:?}", rt.err().unwrap());
            exit(1);
        }
    };

    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "create" => create(&rt),
                "delete" => delete(&rt),
                "edit" => edit(&rt),
                "list" => list(&rt),
                "diary" => diary(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            }
        });
}

fn create(rt: &Runtime) {
    let diaryname = get_diary_name(rt);
    if diaryname.is_none() {
        warn!("No diary selected. Use either the configuration file or the commandline option");
        exit(1);
    }
    let diaryname = diaryname.unwrap();

    let prevent_edit = rt.cli().subcommand_matches("create").unwrap().is_present("no-edit");

    let diary = Diary::open(rt.store(), &diaryname[..]);
    let res = diary.new_entry_today()
        .and_then(|mut entry| {
            if prevent_edit {
                debug!("Not editing new diary entry");
                Ok(())
            } else {
                debug!("Editing new diary entry");
                entry.edit_content(rt)
                    .map_err(|e| DE::new(DEK::DiaryEditError, Some(Box::new(e))))
            }
        });

    if let Err(e) = res {
        trace_error(&e);
    } else {
        info!("Ok!");
    }
}

fn list(rt: &Runtime) {
    let diaryname = get_diary_name(rt);
    if diaryname.is_none() {
        warn!("No diary selected. Use either the configuration file or the commandline option");
        exit(1);
    }
    let diaryname = diaryname.unwrap();

    fn location_to_listing_string(id: &StoreId, base: &PathBuf) -> String {
        id.strip_prefix(base)
            .map_err(|e| trace_error(&e))
            .ok()
            .and_then(|p| p.to_str().map(String::from))
            .unwrap_or(String::from("<<Path Parsing Error>>"))
    }

    let diary = Diary::open(rt.store(), &diaryname[..]);
    debug!("Diary opened: {:?}", diary);
    diary.entries()
        .and_then(|es| {
            debug!("Iterator for listing: {:?}", es);

            let es = es.filter_map(|a| {
                debug!("Filtering: {:?}", a);
                a.ok()
            }).map(|e| e.into());

            let base = rt.store().path();

            CoreLister::new(&move |e| location_to_listing_string(e.get_location(), base))
                .list(es) // TODO: Do not ignore non-ok()s
                .map_err(|e| DE::new(DEK::IOError, Some(Box::new(e))))
        })
        .map(|_| debug!("Ok"))
        .map_err(|e| trace_error(&e))
        .ok();
}

fn delete(rt: &Runtime) {
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

fn edit(rt: &Runtime) {
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
        Some(Ok(mut e)) => e.edit_content(rt).map_err(|e| DE::new(DEK::IOError, Some(Box::new(e)))),

        Some(Err(e)) => Err(e),
        None => Err(DE::new(DEK::EntryNotInDiary, None)),
    }
    .map_err(|e| trace_error(&e)).ok();
}

fn diary(rt: &Runtime) {
    unimplemented!()
}


fn get_diary_name(rt: &Runtime) -> Option<String> {
    use libimagdiary::config::get_default_diary_name;

    get_default_diary_name(rt)
        .or(rt.cli().value_of("diaryname").map(String::from))
}
