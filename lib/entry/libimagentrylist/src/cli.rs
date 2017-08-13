//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

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
