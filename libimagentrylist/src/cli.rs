use clap::{Arg, App, SubCommand};

pub fn build_list_cli_component<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(list_subcommand_name())
        .author("Matthias Beyer <mail@beyermatthias.de>")
        .version("0.1")
        .about("List entries")

        .arg(Arg::with_name(list_backend_line())
             .short("l")
             .long("line")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Use backend: Line"))

        .arg(Arg::with_name(list_backend_path())
             .short("p")
             .long("path")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Use backend: Path"))

        .arg(Arg::with_name(list_backend_path_absolute())
             .short("P")
             .long("path-absolute")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Use backend: Path (absolute)"))

}

pub fn list_subcommand_name() -> &'static str {
    "list"
}

pub fn list_backend_line() -> &'static str {
    "line"
}

pub fn list_backend_path() -> &'static str {
    "path"
}

pub fn list_backend_path_absolute() -> &'static str {
    "path-absolute"
}

