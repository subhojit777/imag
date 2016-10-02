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

use clap::{Arg, ArgGroup, App, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("internal")
                    .about("Add, remove and list internal links")
                    .version("0.1")
                    .subcommand(SubCommand::with_name("add")
                                .about("Add link from one entry to another (and vice-versa)")
                                .version("0.1")
                                .arg(Arg::with_name("from")
                                     .long("from")
                                     .short("f")
                                     .takes_value(true)
                                     .required(true)
                                     .help("Link from this entry")
                                     .value_name("ENTRY"))
                                .arg(Arg::with_name("to")
                                     .long("to")
                                     .short("t")
                                     .takes_value(true)
                                     .required(true)
                                     .multiple(true)
                                     .help("Link to this entries")
                                     .value_name("ENTRIES"))
                                )

                    .subcommand(SubCommand::with_name("remove")
                            .about("Remove a link between two or more entries")
                            .version("0.1")
                            .arg(Arg::with_name("from")
                                 .long("from")
                                 .short("f")
                                 .takes_value(true)
                                 .required(true)
                                 .help("Remove Link from this entry")
                                 .value_name("ENTRY"))
                            .arg(Arg::with_name("to")
                                 .long("to")
                                 .short("t")
                                 .takes_value(true)
                                 .required(true)
                                 .multiple(true)
                                 .help("Remove links to these entries")
                                 .value_name("ENTRIES"))
                            )

                    .arg(Arg::with_name("list")
                         .long("list")
                         .short("l")
                         .takes_value(true)
                         .required(false)
                         .help("List links to this entry")
                         .value_name("ENTRY"))
                    )
        .subcommand(SubCommand::with_name("external")
                    .about("Add and remove external links")
                    .version("0.1")

                    .arg(Arg::with_name("id")
                         .long("id")
                         .short("i")
                         .takes_value(true)
                         .required(true)
                         .help("Modify external link of this entry")
                         .value_name("ENTRY"))

                    .arg(Arg::with_name("add")
                         .long("add")
                         .short("a")
                         .takes_value(true)
                         .required(false)
                         .help("Add this URI as external link")
                         .value_name("URI"))

                    .arg(Arg::with_name("remove")
                         .long("remove")
                         .short("r")
                         .takes_value(false)
                         .required(false)
                         .help("Remove one external link"))

                    .arg(Arg::with_name("set")
                         .long("set")
                         .short("s")
                         .takes_value(true)
                         .required(false)
                         .help("Set these URIs as external link (seperate by comma)")
                         .value_name("URIs"))

                    .arg(Arg::with_name("list")
                         .long("list")
                         .short("l")
                         .takes_value(false)
                         .required(false)
                         .help("List external links"))

                    .group(ArgGroup::with_name("external-link-group")
                           .args(&["add", "remove", "set", "list"])
                           .required(true))

                    )
}
