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
        .arg(Arg::with_name("id")
            .long("id")
            .short("i")
            .takes_value(true)
            .required(true)
            .help("View this entry at this store path")
            .value_name("ID"))

        .arg(Arg::with_name("view-header")
            .long("header")
            .short("h")
            .takes_value(false)
            .required(false)
            .help("View header"))
        .arg(Arg::with_name("view-content")
            .long("content")
            .short("C")
            .takes_value(false)
            .required(false)
            .help("View content"))

        .subcommand(SubCommand::with_name("view-in")
                   .about("View the entry in ...")
                   .version("0.1")

                   .arg(Arg::with_name("view-in-stdout")
                        .long("stdout")
                        .short("s")
                        .takes_value(false)
                        .required(false)
                        .help("View by printing to stdout"))

                   .arg(Arg::with_name("view-in-ui")
                        .long("ui")
                        .short("u")
                        .takes_value(false)
                        .required(false)
                        .help("View by opening own curses-like UI (default)"))

                   .arg(Arg::with_name("view-in-browser")
                        .long("browser")
                        .short("b")
                        .takes_value(true) // optional, which browser
                        .required(false)
                        .help("View content in $BROWSER (fails if no env variable $BROWSER)")
                        .value_name("BROWSER"))

                   .arg(Arg::with_name("view-in-texteditor")
                        .long("in-editor")
                        .short("e")
                        .takes_value(false)
                        .required(false)
                        .help("View content in $EDITOR (can be passed via --editor)"))

                   .arg(Arg::with_name("view-in-custom")
                        .long("custom")
                        .short("c")
                        .takes_value(true) // non-optional, call-string
                        .required(false)
                        .help("View content in custom program, for example 'libreoffice %e', replace '%e' with entry path")
                        .value_name("PROGRAM"))

                   .group(ArgGroup::with_name("viewer")
                          .args(&["view-in-stdout",
                                  "view-in-ui",
                                  "view-in-browser",
                                  "view-in-texteditor",
                                  "view-in-custom",
                          ])
                          .required(false))
                   )

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
