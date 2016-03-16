use clap::{Arg, App, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .arg(Arg::with_name("increment")
             .long("inc")
             .short("i")
             .takes_value(true)
             .required(false)
             .help("Increment a counter"))

        .arg(Arg::with_name("decrement")
             .long("dec")
             .short("d")
             .takes_value(true)
             .required(false)
             .help("Decrement a counter"))

        .arg(Arg::with_name("reset")
             .long("reset")
             .takes_value(true)
             .required(false)
             .help("Reset a counter"))

        .arg(Arg::with_name("set")
             .long("set")
             .takes_value(true)
             .required(false)
             .help("Set a counter"))

        .subcommand(SubCommand::with_name("create")
                   .about("Create a counter")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Create counter with this name"))
                   .arg(Arg::with_name("initval")
                        .long("init")
                        .short("i")
                        .takes_value(true)
                        .required(false)
                        .help("Initial value")))

        .subcommand(SubCommand::with_name("delete")
                   .about("Delete a counter")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Create counter with this name")))
}


