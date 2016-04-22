use clap::{Arg, App, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
       .arg(Arg::with_name("diaryname")
            .long("diary")
            .short("d")
            .takes_value(true)
            .required(false)
            .help("Use other than default diary"))

        .subcommand(SubCommand::with_name("create")
                   .about("Create a diary entry")
                   .version("0.1")
                   .arg(Arg::with_name("no-edit")
                        .long("no-edit")
                        .short("e")
                        .takes_value(false)
                        .required(false)
                        .help("Do not edit after creating"))
                   )

        .subcommand(SubCommand::with_name("edit")
                   .about("Edit a diary entry")
                   .version("0.1")
                   .arg(Arg::with_name("datetime")
                        .long("datetime")
                        .short("d")
                        .takes_value(true)
                        .required(false)
                        .help("Specify the date and time which entry should be edited. If none is
                        specified, the last entry is edited. If the diary entry does not exist for
                        this time, this fails. Format: YYYY-MM-DDT[HH[:mm[:ss]]]"))
                   )

        .subcommand(SubCommand::with_name("list")
                   .about("List diary entries")
                   .version("0.1"))

        // TODO: Support deleting diary entries
        // .subcommand(SubCommand::with_name("delete")
        //            .about("Delete a diary entry")
        //            .version("0.1")
}

