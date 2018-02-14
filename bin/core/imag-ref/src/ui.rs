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
        .subcommand(SubCommand::with_name("deref")
                    .about("'Dereference' a ref. This prints the Path of the referenced file")
                    .version("0.1")
                    .arg(Arg::with_name("ID")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .help("The id of the store entry to dereference")
                         .value_name("ID"))
                    )

        .subcommand(SubCommand::with_name("remove")
                .about("Remove a reference from an entry")
                .version("0.1")
                .arg(Arg::with_name("ID")
                     .index(1)
                     .takes_value(true)
                     .required(true)
                     .multiple(true)
                     .help("Remove the reference from this store entry")
                     .value_name("ENTRIES"))

                .arg(Arg::with_name("yes")
                     .long("yes")
                     .short("y")
                     .help("Don't ask whether this really should be done"))
                )
}
