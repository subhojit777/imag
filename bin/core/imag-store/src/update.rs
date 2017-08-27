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

use std::ops::DerefMut;
use std::path::PathBuf;

use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error_exit;
use libimagstore::storeid::StoreId;

use util::build_toml_header;

pub fn update(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("update")
        .map(|scmd| {
            scmd.value_of("id")
                .map(|id| {
                    let path = PathBuf::from(id);
                    let path = match StoreId::new(Some(rt.store().path().clone()), path) {
                        Err(e) => trace_error_exit(&e, 1),
                        Ok(p) => p,
                    };

                    rt.store()
                        .retrieve(path)
                        .map(|mut locked_e| {
                            let mut e = locked_e.deref_mut();

                            scmd.value_of("content")
                                .map(|new_content| {
                                    *e.get_content_mut() = String::from(new_content);
                                    debug!("New content set");
                                });

                            *e.get_header_mut() = build_toml_header(scmd, e.get_header().clone());
                            debug!("New header set");
                        })
                })
        });

}

