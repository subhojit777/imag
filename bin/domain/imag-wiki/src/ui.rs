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
        .arg(Arg::with_name("wikiname")
                .long("wiki")
                .short("w")
                .takes_value(true)
                .required(false)
                .multiple(false)
                .value_name("WIKI")
                .help("Name of the wiki to use. Defaults to 'default'"))

        .subcommand(SubCommand::with_name("ids")
                   .about("List all ids in this wiki")
                   .version("0.1")

                   .arg(Arg::with_name("ids-full")
                        .long("full")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Print full filepath")))

        .subcommand(SubCommand::with_name("idof")
                   .about("List id of an entry in this wiki, if it exists")
                   .version("0.1")

                   .arg(Arg::with_name("idof-full")
                        .long("full")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Print full filepath"))

                   .arg(Arg::with_name("idof-name")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .multiple(false)
                        .value_name("NAME")
                        .help("Add the entry under this name. The name must be unique, namespaces ('foo/bar') are allowed."))
                   )

        .subcommand(SubCommand::with_name("create-wiki")
                   .about("Create wiki")
                   .version("0.1")
                   .arg(Arg::with_name("create-wiki-name")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .multiple(false)
                        .value_name("NAME")
                        .help("Name of the wiki"))

                   .arg(Arg::with_name("create-wiki-noedit")
                        .long("no-edit")
                        .short("E")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Do not call the editor on the newly created entry.")
                        .conflicts_with("create-wiki-editheader"))

                   .arg(Arg::with_name("create-wiki-editheader")
                        .long("header")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Do edit header when editing main page entry.")
                        .conflicts_with("create-wiki-noedit"))

                   .arg(Arg::with_name("create-wiki-printid")
                        .long("print-id")
                        .short("I")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Print the store id after creating"))
                   )

        .subcommand(SubCommand::with_name("create")
                   .about("Add wiki entry")
                   .version("0.1")

                   .arg(Arg::with_name("create-name")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .multiple(false)
                        .help("Name of the page."))

                   .arg(Arg::with_name("create-noedit")
                        .long("no-edit")
                        .short("E")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Do not call the editor on the newly created entry.")
                        .conflicts_with("create-editheader"))

                   .arg(Arg::with_name("create-editheader")
                        .long("header")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Do edit header when editing entry.")
                        .conflicts_with("create-noedit"))

                   .arg(Arg::with_name("create-printid")
                        .long("print-id")
                        .short("I")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Print the store id after creating"))
                   )

        .subcommand(SubCommand::with_name("show")
                   .about("Show wiki entry/entries")
                   .version("0.1")

                   .arg(Arg::with_name("show-name")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .multiple(true)
                        .help("Name of the entry/entries to show (if not passed, all are shown)."))
                   )


        .subcommand(SubCommand::with_name("delete")
                   .about("Delete wiki entry")
                   .version("0.1")
                   .arg(Arg::with_name("delete-name")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .multiple(false)
                        .value_name("NAME")
                        .help("Delete the entry under this name. The name must be unique, namespaces ('foo/bar') are allowed."))

                   .arg(Arg::with_name("delete-no-remove-linkings")
                        .long("no-remove-links")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Do not remote links. WARNING: This leaves the store in an inconsistent state."))
                   )

}
