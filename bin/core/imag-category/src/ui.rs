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

use clap::{Arg, ArgGroup, App, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("create-category")
                    .about("Create a new category")
                    .version("0.1")
                    .arg(Arg::with_name("create-category-name")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("The name of the new category")
                         .value_name("NAME"))
                   )

        .subcommand(SubCommand::with_name("delete-category")
                    .about("Delete a new category")
                    .version("0.1")
                    .arg(Arg::with_name("delete-category-name")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("The name of the category to delete")
                         .value_name("NAME"))
                   )

        .subcommand(SubCommand::with_name("list-categories")
                    .about("Show all category names")
                    .version("0.1"))

        .subcommand(SubCommand::with_name("list-category")
                    .about("List all entries for a category")
                    .version("0.1")
                    .arg(Arg::with_name("list-category-name")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("The name of the category to list all entries for")
                         .value_name("NAME"))
                   )

        .subcommand(SubCommand::with_name("set")
                    .about("Set the category of entries")
                    .version("0.1")
                    .arg(Arg::with_name("set-name")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("The name of the category to list all entries for")
                         .value_name("NAME"))

                    .arg(Arg::with_name("set-ids")
                         .index(2)
                         .takes_value(true)
                         .required(false)
                         .multiple(true)
                         .help("The entries to set the category for")
                         .value_name("ID"))
                    .arg(Arg::with_name("entries-from-stdin")
                         .long("ids-from-stdin")
                         .short("I")
                         .takes_value(false)
                         .required(false)
                         .multiple(false)
                         .help("Read the ids for the entries from stdin"))

                    .group(ArgGroup::with_name("input-method")
                           .args(&["set-ids", "entries-from-stdin"])
                           .required(true))
                   )

        .subcommand(SubCommand::with_name("get")
                    .about("Get the category of the entry")
                    .version("0.1")
                    .arg(Arg::with_name("get-ids")
                         .index(1)
                         .takes_value(true)
                         .required(false)
                         .multiple(true)
                         .help("The id of the Entry to get the category for")
                         .value_name("ID"))
                    .arg(Arg::with_name("entries-from-stdin")
                         .long("ids-from-stdin")
                         .short("I")
                         .takes_value(false)
                         .required(false)
                         .multiple(false)
                         .help("Read the ids for the entries from stdin"))

                    .group(ArgGroup::with_name("input-method")
                           .args(&["get-ids", "entries-from-stdin"])
                           .required(true))
                   )
}

