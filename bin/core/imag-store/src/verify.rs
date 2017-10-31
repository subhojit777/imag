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

use std::ops::Deref;

use libimagrt::runtime::Runtime;
use libimagutil::warn_exit::warn_exit;
use libimagstore::store::Header;
use libimagstore::store::StoreObject;

/// Verify the store.
///
/// This function is not intended to be called by normal programs but only by `imag-store`.
pub fn verify(rt: &Runtime) {
    use libimagerror::trace::trace_error_dbg;

    info!("Header | Content length | Path");
    info!("-------+----------------+-----");
    let result = rt
        .store()
        .walk("")
        .into_iter()
        .all(|res| match res {
            StoreObject::Collection(_) => true,
            StoreObject::Id(id) => {
                match rt.store().get(id.clone()) {
                    Ok(Some(fle)) => {
                        let p           = fle.get_location();
                        let content_len = fle.get_content().len();
                        let header      = if fle.get_header().verify().is_ok() {
                            "ok"
                        } else {
                            "broken"
                        };

                        info!("{: >6} | {: >14} | {:?}", header, content_len, p.deref());
                        true
                    },

                    Ok(None) => {
                        info!("{: >6} | {: >14} | {:?}", "?", "couldn't load", id.local());
                        true
                    },

                    Err(e) => {
                        trace_error_dbg(&e);
                        false
                    },
                }
            },
        });

    if result {
        info!("Store seems to be fine");
    } else {
        warn_exit("Store seems to be broken somehow", 1);
    }
}

