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

use clap::{Arg, ArgGroup, App};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .arg(Arg::with_name("entry")
             .index(1)
             .takes_value(true)
             .required(false)
             .multiple(true)
             .help("The entry/entries to edit")
             .value_name("ENTRY"))
        .arg(Arg::with_name("entries-from-stdin")
             .long("ids-from-stdin")
             .short("I")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("The entry/entries are piped in via stdin"))
        .group(ArgGroup::with_name("input-method")
               .args(&["entry", "entries-from-stdin"])
               .required(true))

        .arg(Arg::with_name("list-id")
             .long("list-id")
             .short("l")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("List Store Id in output (format: '<id> - <value>'"))
        .arg(Arg::with_name("list-id-format")
             .long("list-id-format")
             .short("L")
             .takes_value(true)
             .required(false)
             .multiple(false)
             .help("List Store Id in output with format"))

        .subcommand(SubCommand::with_name("read")
                    .about("Read a header value by path")
                    .version("0.1")
                    .arg(Arg::with_name("header-value-path")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("Path of the header value")
                         .value_name("PATH"))
                   )

        .subcommand(SubCommand::with_name("has")
                    .about("Check whether a header value is present")
                    .version("0.1")
                    .arg(Arg::with_name("header-value-path")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("Path of the header value")
                         .value_name("PATH"))
                   )

        .subcommand(SubCommand::with_name("hasnt")
                    .about("Check whether a header value is not present")
                    .version("0.1")
                    .arg(Arg::with_name("header-value-path")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("Path of the header value")
                         .value_name("PATH"))
                   )

        .subcommand(SubCommand::with_name("int")
                    .about("Check whether a header value is a number")
                    .version("0.1")
                    .arg(Arg::with_name("header-value-path")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("Path of the header value")
                         .value_name("PATH"))
                   )

        .subcommand(SubCommand::with_name("float")
                    .about("Check whether a header value is a floating number")
                    .version("0.1")
                    .arg(Arg::with_name("header-value-path")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("Path of the header value")
                         .value_name("PATH"))
                   )

        .subcommand(SubCommand::with_name("string")
                    .about("Check whether a header value is a string")
                    .version("0.1")
                    .arg(Arg::with_name("header-value-path")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("Path of the header value")
                         .value_name("PATH"))
                   )

        .subcommand(SubCommand::with_name("bool")
                    .about("Check whether a header value is a bool")
                    .version("0.1")
                    .arg(Arg::with_name("header-value-path")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("Path of the header value")
                         .value_name("PATH"))
                   )

        .subcommand(SubCommand::with_name("exec")
                    .about("Execute a script on the entry (script has only READ access!)")
                    .version("0.1")
                    .arg(Arg::with_name("script")
                         .index(1)
                         .takes_value(true)
                         .required(false)
                         .multiple(false)
                         .help("Script to execute")
                         .value_name("SCRIPT"))
                    .arg(Arg::with_name("scriptfile")
                         .index(1)
                         .takes_value(true)
                         .required(false)
                         .multiple(false)
                         .help("File that contains the script")
                         .value_name("PATH"))
                    .group(ArgGroup::with_name("script-text")
                           .args(&["script", "scriptfile"])
                           .required(true))

                    .arg(Arg::with_name("scriptlang-lua")
                         .long("lua")
                         .takes_value(false)
                         .required(false)
                         .multiple(false)
                         .help("Hint that the script is a LUA script"))

                    .arg(Arg::with_name("scriptlang-ketos")
                         .long("ketos")
                         .takes_value(false)
                         .required(false)
                         .multiple(false)
                         .help("Hint that the script is a KETOS script"))

                    .arg(Arg::with_name("scriptlang-dyon")
                         .long("dyon")
                         .takes_value(false)
                         .required(false)
                         .multiple(false)
                         .help("Hint that the script is a DYON script"))
                   )

}
