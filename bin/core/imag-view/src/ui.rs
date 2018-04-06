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
        .arg(Arg::with_name("entries-from-stdin")
             .long("ids-from-stdin")
             .short("I")
             .required(false)
             .multiple(true)
             .help("The entry/entries are piped in via stdin"))

        .arg(Arg::with_name("id")
             .index(1)
             .takes_value(true)
             .required(false)
             .multiple(true)
             .help("View these entries at this store path")
             .value_name("IDs"))

        .group(ArgGroup::with_name("input-method")
               .args(&["id", "entries-from-stdin"])
               .required(true))

        .arg(Arg::with_name("view-header")
            .long("header")
            .short("h")
            .takes_value(false)
            .required(false)
            .help("View header"))
        .arg(Arg::with_name("not-view-content")
            .long("no-content")
            .short("C")
            .takes_value(false)
            .required(false)
            .help("Do not view content"))

        .arg(Arg::with_name("in")
            .long("in")
            .takes_value(true)
            .required(false)
            .multiple(false)
            .help("View content. If no value is given, this fails. Possible viewers are configured via the config file."))

        .subcommand(SubCommand::with_name("compile")
                   .about("Compile content to other format for viewing, implies that the entry gets copied to /tmp")
                   .version("0.1")
                   .arg(Arg::with_name("from")
                        .long("from")
                        .short("f")
                        .takes_value(true) // "markdown" or "textile" or "restructuredtex"
                        .required(true)
                        .help("Compile from")
                        .value_name("FORMAT"))

                   .arg(Arg::with_name("to")
                        .long("to")
                        .short("t")
                        .takes_value(true) // "html" or "HTML" or ... json maybe?
                        .required(true)
                        .help("Compile to")
                        .value_name("FORMAT"))
                   )
}
