use clap::{Arg, ArgGroup, App, SubCommand};

use libimagtag::ui::tag_argument;
use libimagtag::ui::tag_argument_name;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("create")
                   .about("Create a note")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Create Note with this name"))
                   .arg(Arg::with_name("edit")
                        .long("edit")
                        .short("e")
                        .takes_value(false)
                        .required(false)
                        .help("Edit after creating"))
                   )

        .subcommand(SubCommand::with_name("delete")
                   .about("Delete a Note")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Delete Note with this name")))

        .subcommand(SubCommand::with_name("edit")
                   .about("Edit a Note")
                   .version("0.1")
                   .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .required(true)
                        .help("Edit Note with this name"))

                   .arg(tag_argument())
                   .group(ArgGroup::with_name("editargs")
                          .args(&[tag_argument_name(), "name"])
                          .required(true))
                   )

        .subcommand(SubCommand::with_name("list")
                   .about("List Notes")
                   .version("0.1"))

}


