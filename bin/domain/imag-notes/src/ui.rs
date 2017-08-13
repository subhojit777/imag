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

use clap::{Arg, App, SubCommand};

use libimagentrytag::ui::tag_argument;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("create")
                   .about("Create a note")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Create Note with this name")
                        .value_name("NAME"))
                   .arg(Arg::with_name("edit")
                        .long("edit")
                        .short("e")
                        .takes_value(false)
                        .required(false)
                        .help("Edit after creating"))
                   )

        .subcommand(SubCommand::with_name("delete")
                   .about("Delete a Note")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Delete Note with this name")
                        .value_name("NAME")))

        .subcommand(SubCommand::with_name("edit")
                   .about("Edit a Note")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Edit Note with this name")
                        .value_name("NAME"))

                   .arg(tag_argument())
                   )

        .subcommand(SubCommand::with_name("list")
                   .about("List Notes")
                   .version("0.1"))

}
