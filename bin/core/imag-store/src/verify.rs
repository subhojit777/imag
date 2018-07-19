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

use std::ops::Deref;

use libimagrt::runtime::Runtime;
use libimagutil::warn_exit::warn_exit;
use libimagerror::trace::MapErrTrace;
use libimagerror::iter::TraceIterator;

/// Verify the store.
///
/// This function is not intended to be called by normal programs but only by `imag-store`.
pub fn verify(rt: &Runtime) {
    info!("Header | Content length | Path");
    info!("-------+----------------+-----");
    let result = rt
        .store()
        .entries()
        .map_err_trace_exit_unwrap(1)
        .into_get_iter()
        .trace_unwrap_exit(1)
        .filter_map(|x| x)
        .all(|fle| {
            let p           = fle.get_location();
            let content_len = fle.get_content().len();
            let (verify, status) = if fle.verify().is_ok() {
                ("ok", true)
            } else {
                ("broken", false)
            };

            info!("{: >6} | {: >14} | {:?}", verify, content_len, p.deref());
            status
        });

    if result {
        info!("Store seems to be fine");
    } else {
        warn_exit("Store seems to be broken somehow", 1);
    }
}

