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
                         .help("Args written in the string will be send directly to taskwarrior"))

                    )

} 
