use libimagdiary::diary::Diary;
use libimagdiary::viewer::DiaryViewer as DV;
use libimagrt::runtime::Runtime;
use libimagerror::trace::MapErrTrace;
use libimagutil::warn_exit::warn_exit;

use util::get_diary_name;

pub fn view(rt: &Runtime) {
    let diaryname = get_diary_name(rt).unwrap_or_else(|| warn_exit("No diary name", 1));
    let diary     = Diary::open(rt.store(), &diaryname[..]);
    let hdr       = rt.cli().subcommand_matches("view").unwrap().is_present("show-header");

    diary.entries()
        .and_then(|entries| DV::new(hdr).view_entries(entries.into_iter().filter_map(Result::ok)))
        .map_err_trace()
        .ok();
}

