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

use std::io::Write;

use libimagrt::runtime::Runtime;
use libimagerror::trace::*;
use libimagerror::io::ToExitCode;
use libimagerror::exit::ExitUnwrap;

pub fn ids(rt: &Runtime) {
    let full = rt.cli().subcommand_matches("ids").unwrap() //secured by main
        .is_present("full");
    let base = rt.store().path();
    let _ = rt
        .store()
        .entries()
        .map_err_trace_exit_unwrap(1)
        .map(|e| if full {
            e.with_base(base.clone())
        } else {
           e.without_base()
        })
        .map(|i| i.to_str())
        .map(|elem| elem.map_err_trace_exit_unwrap(1))
        .map(|i| writeln!(::std::io::stdout(), "{}", i))
        .collect::<Result<Vec<()>, ::std::io::Error>>()
        .to_exit_code()
        .unwrap_or_exit();
}

