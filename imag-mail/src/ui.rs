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
        .subcommand(SubCommand::with_name("import-mail")
                    .about("Import a mail (create a reference to it) (Maildir)")
                    .version("0.1")
                    .arg(Arg::with_name("path")
                         .long("path")
                         .short("p")
                         .takes_value(true)
                         .required(true)
                         .help("Path to the mail file or a directory which is then searched recursively")
                         .value_name("PATH"))
                    )

        .subcommand(SubCommand::with_name("list")
                    .about("List all stored references to mails")
                    .version("0.1")

                    // TODO: Thee following four arguments are the same as in imag-ref.
                    // We should make these importable from libimagref.

                    .arg(Arg::with_name("check-dead")
                         .long("check-dead")
                         .short("d")
                         .help("Check each reference whether it is dead"))

                    .arg(Arg::with_name("check-changed")
                         .long("check-changed")
                         .short("c")
                         .help("Check whether a reference had changed (content or permissions)"))

                    .arg(Arg::with_name("check-changed-content")
                         .long("check-changed-content")
                         .short("C")
                         .help("Check whether the content of the referenced file changed"))

                    .arg(Arg::with_name("check-changed-permissions")
                         .long("check-changed-perms")
                         .short("P")
                         .help("Check whether the permissions of the referenced file changed"))

                    )

        .subcommand(SubCommand::with_name("mail-store")
                    .about("Operations on (subsets of) all mails")
                    .version("0.1")
                    .subcommand(SubCommand::with_name("update-refs")
                                .about("Create references based on Message-IDs for all loaded mails")
                                .version("0.1"))
                    // TODO: We really should be able to filter here.
                    )
}

