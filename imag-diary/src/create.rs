use std::process::exit;

use libimagdiary::diary::Diary;
use libimagdiary::diaryid::DiaryId;
use libimagdiary::error::DiaryError as DE;
use libimagdiary::error::DiaryErrorKind as DEK;
use libimagrt::edit::Edit;
use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error;
use libimagdiary::entry::Entry;
use libimagdiary::result::Result;

use util::get_diary_name;

pub fn create(rt: &Runtime) {

    let diaryname = get_diary_name(rt);
    if diaryname.is_none() {
        warn!("No diary selected. Use either the configuration file or the commandline option");
        exit(1);
    }
    let diaryname = diaryname.unwrap();

    let prevent_edit = rt.cli().subcommand_matches("create").unwrap().is_present("no-edit");

    fn create_entry<'a>(diary: &'a Diary, rt: &Runtime) -> Result<Entry<'a>> {
        use std::str::FromStr;

        let create = rt.cli().subcommand_matches("create").unwrap();
        if !create.is_present("timed") {
            debug!("Creating non-timed entry");
            diary.new_entry_today()
        } else {
            let id = match create.value_of("timed") {
                Some("h") | Some("hourly") => {
                    debug!("Creating hourly-timed entry");
                    let time = DiaryId::now(String::from(diary.name()));
                    let hr = create
                        .value_of("hour")
                        .map(|v| { debug!("Creating hourly entry with hour = {:?}", v); v })
                        .and_then(|s| {
                            FromStr::from_str(s)
                                .map_err(|_| warn!("Could not parse hour: '{}'", s))
                                .ok()
                        })
                        .unwrap_or(time.hour());

                    time.with_hour(hr).with_minute(0)
                },

                Some("m") | Some("minutely") => {
                    debug!("Creating minutely-timed entry");
                    let time = DiaryId::now(String::from(diary.name()));
                    let hr = create
                        .value_of("hour")
                        .map(|h| { debug!("hour = {:?}", h); h })
                        .and_then(|s| {
                            FromStr::from_str(s)
                                .map_err(|_| warn!("Could not parse hour: '{}'", s))
                                .ok()
                        })
                        .unwrap_or(time.hour());

                    let min = create
                        .value_of("minute")
                        .map(|m| { debug!("minute = {:?}", m); m })
                        .and_then(|s| {
                            FromStr::from_str(s)
                                .map_err(|_| warn!("Could not parse minute: '{}'", s))
                                .ok()
                        })
                        .unwrap_or(time.minute());

                    time.with_hour(hr).with_minute(min)
                },

                Some(_) => {
                    warn!("Timed creation failed: Unknown spec '{}'",
                          create.value_of("timed").unwrap());
                    exit(1);
                },

                None => {
                    warn!("Unexpected error, cannot continue");
                    exit(1);
                },
            };

            diary.new_entry_by_id(id)
        }
    }

    let diary = Diary::open(rt.store(), &diaryname[..]);
    let res = create_entry(&diary, rt)
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

