use std::process::exit;

use libimagdiary::diary::Diary;
use libimagdiary::error::DiaryErrorKind as DEK;
use libimagdiary::error::MapErrInto;
use libimagentrylist::listers::core::CoreLister;
use libimagentrylist::lister::Lister;
use libimagrt::runtime::Runtime;
use libimagstore::store::Entry;
use libimagerror::trace::trace_error;

use util::get_diary_name;

pub fn list(rt: &Runtime) {
    let diaryname = get_diary_name(rt).unwrap_or_else(|| {
        warn!("No diary selected. Use either the configuration file or the commandline option");
        exit(1);
    });

    fn entry_to_location_listing_string(e: &Entry) -> String {
        e.get_location().clone()
            .without_base()
            .to_str()
            .map_err(|e| trace_error(&e))
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

            CoreLister::new(&entry_to_location_listing_string)
                .list(es) // TODO: Do not ignore non-ok()s
                .map_err_into(DEK::IOError)
        })
        .map(|_| debug!("Ok"))
        .map_err(|e| trace_error(&e))
        .ok();
}

