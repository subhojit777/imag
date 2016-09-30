use clap::{Arg, App, SubCommand};

use libimagutil::cli_validators::is_existing_path;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("add")
                    .about("Add a reference to a file outside of the store")
                    .version("0.1")
                    .arg(Arg::with_name("path")
                         .long("path")
                         .short("p")
                         .takes_value(true)
                         .required(true)
                         .help("The path of the file")
                         .validator(is_existing_path)
                         .value_name("PATH"))
                    .arg(Arg::with_name("track-content")
                         .long("content-hash")
                         .short("C")
                         .takes_value(false)
                         .required(false)
                         .help("Hash the content for the reference"))
                    .arg(Arg::with_name("track-permissions")
                         .long("permission-tracking")
                         .short("P")
                         .takes_value(false)
                         .required(false)
                         .help("Rememeber the permissions of the referenced file"))
                    )

        .subcommand(SubCommand::with_name("remove")
                .about("Remove a reference")
                .version("0.1")
                .arg(Arg::with_name("hash")
                     .long("hash")
                     .short("h")
                     .takes_value(true)
                     .required(true)
                     .help("Remove the reference with this hash")
                     .value_name("HASH"))

                .arg(Arg::with_name("yes")
                     .long("yes")
                     .short("y")
                     .help("Don't ask whether this really should be done"))
                )

        .subcommand(SubCommand::with_name("list")
                    .about("List references in the store")
                    .version("0.1")

                    .arg(Arg::with_name("check-dead")
                         .long("check-dead")
                         .short("d")
                         .help("Check each reference whether it is dead"))

                    .arg(Arg::with_name("check-changed")
                         .long("check-changed")
                         .short("c")
                         .help("Check whether a reference had changed (content or permissions)"))

                    .arg(Arg::with_name("check-changed-content")
                         .long("check-changed-content")
                         .short("C")
                         .help("Check whether the content of the referenced file changed"))

                    .arg(Arg::with_name("check-changed-permissions")
                         .long("check-changed-perms")
                         .short("P")
                         .help("Check whether the permissions of the referenced file changed"))

                    )
}
