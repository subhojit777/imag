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

use clap::{Arg, App, ArgGroup, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("tw-hook")
                    .about("For use in a taskwarrior hook")
                    .version("0.1")

                    .arg(Arg::with_name("add")
                         .long("add")
                         .short("a")
                         .takes_value(false)
                         .required(false)
                         .help("For use in an on-add hook"))

                    .arg(Arg::with_name("delete")
                         .long("delete")
                         .short("d")
                         .takes_value(false)
                         .required(false)
                         .help("For use in an on-delete hook"))

                    .group(ArgGroup::with_name("taskwarrior hooks")
                           .args(&[ "add",
                                 "delete",
                           ])
                           .required(true))
                    )

        .subcommand(SubCommand::with_name("list")
                    .about("List all tasks")
                    .version("0.1")

                    .arg(Arg::with_name("verbose")
                         .long("verbose")
                         .short("v")
                         .takes_value(false)
                         .required(false)
                         .help("Asks taskwarrior for all the details")
                        )
                   )
}
