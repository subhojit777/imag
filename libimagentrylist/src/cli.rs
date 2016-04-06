use clap::{Arg, ArgMatches, App, SubCommand};

use libimagstore::store::FileLockEntry;

use result::Result;
use listers::line::LineLister;
use listers::path::PathLister;
use lister::Lister;
use error::{ListError, ListErrorKind};

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

// TODO: Add Registry for listers where a HashMap name->lister is in and where we can fetch the
// lister from.
pub fn list_entries_with_lister<'a, I>(m: &ArgMatches, entries: I) -> Result<()>
    where I: Iterator<Item = FileLockEntry<'a>>
{
    if let Some(matches) = m.subcommand_matches(list_subcommand_name()) {
        if matches.is_present(list_backend_line()) {
            return LineLister::new("<unknown>").list(entries)
        };

        if matches.is_present(list_backend_path()) {
            return PathLister::new(false).list(entries)
        }


        if matches.is_present(list_backend_path_absolute()) {
            return PathLister::new(true).list(entries)
        }

        Ok(())
    } else {
        Err(ListError::new(ListErrorKind::CLIError, None))
    }
}
