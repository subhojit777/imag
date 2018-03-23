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

use clap::{Arg, App, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("remove")
                .about("Remove a link between two or more entries")
                .version("0.1")
                .arg(Arg::with_name("from")
                     .index(1)
                     .takes_value(true)
                     .required(true)
                     .multiple(false)
                     .help("Remove Link from this entry")
                     .value_name("ENTRY"))
                .arg(Arg::with_name("to")
                     .index(2)
                     .takes_value(true)
                     .required(true)
                     .multiple(true)
                     .help("Remove links to these entries")
                     .value_name("ENTRIES"))
                )
        .subcommand(SubCommand::with_name("unlink")
                .about("Remove all links from an entry")
                .version("0.1")
                .arg(Arg::with_name("from")
                     .index(1)
                     .takes_value(true)
                     .required(true)
                     .multiple(true)
                     .help("Remove links from these entries")
                     .value_name("ENTRY"))
                )

        .subcommand(SubCommand::with_name("list")
                .about("List links to this entry")
                .version("0.1")
                .arg(Arg::with_name("entries")
                     .index(1)
                     .takes_value(true)
                     .multiple(true)
                     .required(true)
                     .help("List these entries, seperate by comma")
                     .value_name("ENTRIES"))

                .arg(Arg::with_name("list-externals-too")
                     .long("list-external")
                     .takes_value(false)
                     .required(false)
                     .help("Also list external links (debugging helper that might be removed at some point"))

                .arg(Arg::with_name("list-plain")
                     .long("plain")
                     .multiple(false)
                     .takes_value(false)
                     .required(false)
                     .help("List plain rather than in ASCII table"))
                )

        .arg(Arg::with_name("check-consistency")
             .long("check-consistency")
             .short("C")
             .takes_value(false)
             .required(false)
             .help("Check the link-consistency in the store (might be time-consuming)"))

        .arg(Arg::with_name("from")
             .index(1)
             .takes_value(true)
             .required(false)
             .multiple(false)
             .help("Link from this entry")
             .requires("to")
             .value_name("ENTRY"))

        .arg(Arg::with_name("to")
             .index(2)
             .takes_value(true)
             .required(false)
             .multiple(true)
             .help("Link to this entries")
             .requires("from")
             .value_name("ENTRIES"))
}
