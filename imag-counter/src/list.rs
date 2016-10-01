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

use libimagrt::runtime::Runtime;
use libimagerror::trace::{MapErrTrace, trace_error};
use libimagcounter::counter::Counter;

pub fn list(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("list")
        .map(|_| {
            debug!("Found 'list' subcommand...");

            Counter::all_counters(rt.store()).map(|iterator| {
                for counter in iterator {
                    counter.map(|c| {
                        let name    = c.name();
                        let value   = c.value();
                        let unit    = c.unit();

                        if name.is_err() {
                            trace_error(&name.unwrap_err());
                        } else if value.is_err() {
                            trace_error(&value.unwrap_err());
                        } else if unit.is_none() {
                            println!("{} - {}", name.unwrap(), value.unwrap());
                        } else {
                            println!("{} - {} {}", name.unwrap(), value.unwrap(), unit.unwrap());
                        }
                    })
                    .map_err_trace()
                    .ok();
                }
            })
            .map_err_trace()

        });
}
