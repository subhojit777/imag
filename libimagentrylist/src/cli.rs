use clap::{Arg, App, SubCommand};

pub fn build_list_cli_component<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(list_subcommand_name())
        .author("Matthias Beyer <mail@beyermatthias.de>")
        .version("0.1")
        .about("List entries")
}

pub fn list_subcommand_name() -> &'static str {
    "list"
}

