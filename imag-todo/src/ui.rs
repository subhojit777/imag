use clap::{Arg, App, ArgGroup, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("tw-hook")
                    .about("For use in a taskwarrior hook")
                    .version("0.1")

                    .arg(Arg::with_name("add")
                         .long("add")
                         .short("a")
                         .takes_value(false)
                         .required(false)
                         .help("For use in an on-add hook"))

                    .arg(Arg::with_name("delete")
                         .long("delete")
                         .short("d")
                         .takes_value(false)
                         .required(false)
                         .help("For use in an on-delete hook"))

                    .group(ArgGroup::with_name("taskwarrior hooks")
                           .args(&[ "add",
                                 "delete",
                           ])
                           .required(true))
                    )

                    .subcommand(SubCommand::with_name("exec")
                                .about("Send a command to taskwarrior")
                                .version("0.1")

                                .arg(Arg::with_name("command")
                                     .long("command")
                                     .short("c")
                                     .takes_value(true)
                                     .multiple(true)
                                     .required(true)
                                     .help("Args written in the string will be send directly to taskwarrior")
                                    )
                               )

                    .subcommand(SubCommand::with_name("add")
                                .about("create a task")
                                .version("0.1")

                                .arg(Arg::with_name("description")
                                     .long("description")
                                     .short("d")
                                     .takes_value(true)
                                     .required(true)
                                     .help("Name/Description of the new task")
                                    )

                                .arg(Arg::with_name("priority")
                                     .long("priority")
                                     .short("p")
                                     .takes_value(true)
                                     .required(false)
                                     .help("One of l, m, h for low, medium and high priority")
                                    )

                                .arg(Arg::with_name("project")
                                     .long("project")
                                     .takes_value(true)
                                     .required(false)
                                     .help("Name of the project the task is related to")
                                    )

                                .arg(Arg::with_name("due")
                                     .long("due")
                                     .takes_value(true)
                                     .required(false)
                                     .help("Due date of the new task")
                                    )

                                .arg(Arg::with_name("frequency")
                                     .long("frequency")
                                     .short("f")
                                     .takes_value(true)
                                     .required(false)
                                     .help("Frequency of the recurrence of a task")
                                    )
                                )

                    .subcommand(SubCommand::with_name("list")
                                .about("List all tasks")
                                .version("0.1")

                                .arg(Arg::with_name("verbose")
                                     .long("verbose")
                                     .short("v")
                                     .takes_value(false)
                                     .required(false)
                                     .help("Asks taskwarrior for all the details")
                                    )
                               )
}
