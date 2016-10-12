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

