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

use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagerror::trace::*;

pub fn dump(rt: &mut Runtime) {
    let cachingres = rt
        .store()
        .entries()
        .map_err_trace()
        .map(|iter| {
            for elem in iter {
                debug!("Working on {:?}", elem);
                if let Ok(_) = rt.store().get(elem.clone()).map_err_dbg_trace() {
                    info!("Loading entry at {:?} succeeded", elem);
                } else {
                    error!("Loading entry at {:?} failed", elem);
                }
            }
        });

    if let Ok(_) = cachingres {
        if let Err(_) = rt.store_backend_to_stdio().map_err_trace() {
            error!("Loading Store IO backend failed");
            exit(1);
        }
    } else {
        error!("Loading entries failed");
        exit(1);
    }
}

