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
       .arg(Arg::with_name("diaryname")
            .long("to")
            .short("t")
            .takes_value(true)
            .required(false)
            .help("Use other than default diary"))

       .arg(Arg::with_name("text")
            .index(1)
            .takes_value(true)
            .multiple(true)
            .required(false)
            .help("Text to log. Multiple will be concatenated")
            .value_name("TEXT"))

        .subcommand(SubCommand::with_name("show")
                   .about("View log(s)")
                   .version("0.1")

                   .arg(Arg::with_name("show-all")
                        .long("all")
                        .short("a")
                        .takes_value(false)
                        .required(false)
                        .conflicts_with("show-name")
                        .help("Show all logs"))

                   .arg(Arg::with_name("show-name")
                        .index(1)
                        .takes_value(true)
                        .multiple(true)
                        .required(false)
                        .conflicts_with("show-all")
                        .help("Show logs. Multiple possible (will be sorted by date, still)."))

                )

}

