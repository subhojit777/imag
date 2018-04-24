//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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
use libimagstore::iter::get::StoreIdGetIteratorExtension;
use libimagentryview::viewer::Viewer;

use util::get_diary_name;

pub fn view(rt: &Runtime) {
    let diaryname = get_diary_name(rt).unwrap_or_else(|| warn_exit("No diary name", 1));
    let hdr       = rt.cli().subcommand_matches("view").unwrap().is_present("show-header");

    let entries = Diary::entries(rt.store(), &diaryname)
        .map_err_trace_exit_unwrap(1)
        .into_get_iter(rt.store())
        .filter_map(Result::ok)
        .map(|e| e.unwrap_or_else(|| {
            error!("Failed to fetch entry");
            ::std::process::exit(1)
        }));

    let out = rt.stdout();
    DV::new(hdr).view_entries(entries, &mut out.lock())
        .map_err_trace_exit_unwrap(1);
}

