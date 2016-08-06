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
