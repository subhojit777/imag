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

use std::path::PathBuf;
use std::io::Write;

use clap::ArgMatches;

use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagrt::runtime::Runtime;
use libimagerror::trace::MapErrTrace;
use libimagerror::io::ToExitCode;
use libimagerror::exit::ExitUnwrap;
use libimagutil::debug_result::*;

pub fn retrieve(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("retrieve")
        .map(|scmd| {
            // unwrap() is safe as arg is required
            let id    = scmd.value_of("id").unwrap();
            let path  = PathBuf::from(id);
            let store = Some(rt.store().path().clone());
            let path  = StoreId::new(store, path).map_err_trace_exit_unwrap(1);
            debug!("path = {:?}", path);

            rt.store()
                .retrieve(path)
                .map(|e| print_entry(rt, scmd, e))
                .map_dbg_str("No entry")
                .map_dbg(|e| format!("{:?}", e))
                .map_err_trace()
        });
}

pub fn print_entry(rt: &Runtime, scmd: &ArgMatches, e: FileLockEntry) {
    if do_print_raw(scmd) {
        debug!("Printing raw content...");
        let _ = writeln!(rt.stdout(), "{}", e.to_str().map_err_trace_exit_unwrap(1))
            .to_exit_code()
            .unwrap_or_exit();
    } else if do_filter(scmd) {
        debug!("Filtering...");
        warn!("Filtering via header specs is currently now supported.");
        warn!("Will fail now!");
        unimplemented!()
    } else {
        debug!("Printing structured...");
        if do_print_header(scmd) {
            debug!("Printing header...");
            if do_print_header_as_json(rt.cli()) {
                debug!("Printing header as json...");
                warn!("Printing as JSON currently not supported.");
                warn!("Will fail now!");
                unimplemented!()
            } else {
                debug!("Printing header as TOML...");
                let _ = writeln!(rt.stdout(), "{}", e.get_header())
                    .to_exit_code()
                    .unwrap_or_exit();
            }
        }

        if do_print_content(scmd) {
            debug!("Printing content...");
            let _ = writeln!(rt.stdout(), "{}", e.get_content())
                    .to_exit_code()
                    .unwrap_or_exit();
        }

    }
}

fn do_print_header(m: &ArgMatches) -> bool {
    m.is_present("header")
}

fn do_print_header_as_json(m: &ArgMatches) -> bool {
    m.is_present("header-json")
}

fn do_print_content(m: &ArgMatches) -> bool {
    m.is_present("content")
}

fn do_print_raw(m: &ArgMatches) -> bool {
    m.is_present("raw")
}

fn do_filter(m: &ArgMatches) -> bool {
    m.subcommand_matches("filter-header").is_some()
}

