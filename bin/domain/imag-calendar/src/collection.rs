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

pub fn collection(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("collection").unwrap(); // safed by main()

    scmd.subcommand_name()
        .map(|name| match name {
            "add" => add(scmd),
            "remove" => remove(scmd),
            "show" => show(scmd),
            "list" => list(scmd),
            "find" => find(scmd),
            _ => {
                unimplemented!()
            }
        })
        .unwrap_or_else(|| unreachable!("BUG, please report"))
}

fn add<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    let name = scmd.value_of("collection-add-name").map(String::from).unwrap(); // safe by clap
    let path = scmd.value_of("collection-add-path").map(PathBuf::from).unwrap(); // safe by clap

    rt.store()
        .retrieve_calendar_collection(path)
        .map_err_trace_unwrap_exit(1)
}

fn remove<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    unimplemented!()
}

fn show<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    unimplemented!()
}

fn list<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    unimplemented!()
}

fn find<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    unimplemented!()
}
