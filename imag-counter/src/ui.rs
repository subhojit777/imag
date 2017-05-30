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

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .arg(Arg::with_name("increment")
             .long("inc")
             .short("i")
             .takes_value(true)
             .required(false)
             .help("Increment a counter")
             .value_name("COUNTER"))

        .arg(Arg::with_name("decrement")
             .long("dec")
             .short("d")
             .takes_value(true)
             .required(false)
             .help("Decrement a counter")
             .value_name("COUNTER"))

        .arg(Arg::with_name("reset")
             .long("reset")
             .takes_value(true)
             .required(false)
             .help("Reset a counter")
             .value_name("COUNTER"))

        .arg(Arg::with_name("set")
             .long("set")
             .takes_value(true)
             .required(false)
             .help("Set a counter")
             .value_name("COUNTER"))

        .subcommand(SubCommand::with_name("create")
                   .about("Create a counter")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Create counter with this name")
                        .value_name("NAME"))
                   .arg(Arg::with_name("initval")
                        .long("init")
                        .short("i")
                        .takes_value(true)
                        .required(false)
                        .help("Initial value")
                        .value_name("VALUE"))
                    .arg(Arg::with_name("unit")
                        .long("unit")
                        .short("u")
                        .takes_value(true)
                        .required(false)
                        .help("measurement unit")
                        .value_name("UNIT")))

        .subcommand(SubCommand::with_name("delete")
                   .about("Delete a counter")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Create counter with this name")
                        .value_name("NAME")))

        .subcommand(SubCommand::with_name("list")
                   .about("List counters")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(false)
                        .help("List counters with this name (foo/bar and baz/bar would match 'bar')")
                        .value_name("NAME"))

                   .arg(Arg::with_name("greater-than")
                        .long("greater")
                        .short("g")
                        .takes_value(true)
                        .required(false)
                        .help("List counters which are greater than VALUE")
                        .value_name("VALUE"))

                   .arg(Arg::with_name("lower-than")
                        .long("lower")
                        .short("l")
                        .takes_value(true)
                        .required(false)
                        .help("List counters which are lower than VALUE")
                        .value_name("VALUE"))

                   .arg(Arg::with_name("equals")
                        .long("equal")
                        .short("e")
                        .takes_value(true)
                        .required(false)
                        .help("List counters which equal VALUE")
                        .value_name("VALUE"))
        )

        .subcommand(SubCommand::with_name("interactive")
                   .about("Interactively count things")
                   .version("0.1")
                   .arg(Arg::with_name("spec")
                        .long("spec")
                        .short("s")
                        .takes_value(true)
                        .multiple(true)
                        .required(true)
                        .help("Specification for key-bindings. Use <KEY>=<VALUE> where KEY is the
                        key to bind (single character) and VALUE is the path to the counter to bind
                        to.")
                        .value_name("KEY=VALUE")))
}
