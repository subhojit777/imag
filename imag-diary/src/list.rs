use std::path::PathBuf;
use std::process::exit;

use libimagdiary::diary::Diary;
use libimagdiary::error::DiaryError as DE;
use libimagdiary::error::DiaryErrorKind as DEK;
use libimagentrylist::listers::core::CoreLister;
use libimagentrylist::lister::Lister;
use libimagrt::runtime::Runtime;
use libimagstore::storeid::StoreId;
use libimagerror::trace::trace_error;

use util::get_diary_name;

pub fn list(rt: &Runtime) {
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

