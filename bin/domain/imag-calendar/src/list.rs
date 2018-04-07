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

use filters::filter::Filter;

use libimagrt::runtime::Runtime;
use util::PastFilter;

pub fn list(rt: &Runtime) {
    let scmd      = rt.cli().subcommand_matches("list").unwrap();
    let list_past = scmd.is_present("list-past");
    let list_tabl = scmd.is_present("list-table");

    let today = ::chrono::offset::Local::today()
        .and_hms_opt(0, 0, 0)
        .unwrap_or_else(|| {
            error!("BUG, please report");
            ::std::process::exit(1)
        })
        .naive_local();

    let past_filter = PastFilter::new(list_past, today);
    let events      = ::util::all_events(rt.store()).filter(|e| past_filter.filter(e));
    ::util::list_events(rt, list_tabl, events);
}

