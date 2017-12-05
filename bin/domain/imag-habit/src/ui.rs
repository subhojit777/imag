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
        .subcommand(SubCommand::with_name("status")
                   .about("Show the current status. Remind of not-yet-done habits, shows upcoming. Default if no command is passed.")
                   .version("0.1")
                   )

        .subcommand(SubCommand::with_name("create")
                   .about("Create a new Habit")
                   .version("0.1")
                   .arg(Arg::with_name("create-name")
                        .index(1)
                        .multiple(false)
                        .required(true)
                        .takes_value(true)
                        .value_name("NAME")
                        .help("Name of the new habit"))
                   .arg(Arg::with_name("create-date")
                        .index(2)
                        .multiple(false)
                        .required(true)
                        .takes_value(true)
                        .value_name("DATE")
                        .help("Date when the first instance should be done"))
                   .arg(Arg::with_name("create-date-recurr-spec")
                        .index(3)
                        .multiple(false)
                        .required(true)
                        .takes_value(true)
                        .value_name("RECURRENCE-SPEC")
                        .help("Spec how the habit should recur (eg: 'weekly', 'monthly', '5days', '12hours')"))
                   .arg(Arg::with_name("create-comment")
                        .index(4)
                        .multiple(true)
                        .required(true)
                        .takes_value(true)
                        .value_name("COMMENT")
                        .help("Comment for the habit"))
                   )

        .subcommand(SubCommand::with_name("delete")
                   .about("Delete a Habit (and its instances)")
                   .version("0.1")
                   .arg(Arg::with_name("delete-instances")
                        .long("instances")
                        .short("I")
                        .multiple(false)
                        .required(false)
                        .takes_value(false)
                        .help("Delete instances as well"))
                   .arg(Arg::with_name("delete-yes")
                        .long("yes")
                        .multiple(false)
                        .required(false)
                        .takes_value(false)
                        .help("Do not ask for confirmation"))
                   .arg(Arg::with_name("delete-name")
                        .index(1)
                        .multiple(false)
                        .required(true)
                        .takes_value(true)
                        .value_name("NAME")
                        .help("Name of the habit"))
                   )

        .subcommand(SubCommand::with_name("list")
                   .about("List Habits")
                   .version("0.1")
                   .arg(Arg::with_name("list-long")
                        .long("long")
                        .short("l")
                        .multiple(false)
                        .required(false)
                        .takes_value(false)
                        .help("List with details (how many instances)"))
                   )

        .subcommand(SubCommand::with_name("show")
                   .about("Show a Habit and its instances")
                   .version("0.1")
                   .arg(Arg::with_name("show-name")
                        .index(1)
                        .multiple(false)
                        .required(true)
                        .takes_value(true)
                        .value_name("NAME")
                        .help("Name of the habit to show"))
                   )

        .subcommand(SubCommand::with_name("today")
                   .about("List habits which are due today")
                   .version("0.1")
                   )
}
