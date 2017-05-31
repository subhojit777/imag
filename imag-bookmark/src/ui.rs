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

use clap::{Arg, App, SubCommand};

use libimagentrytag::ui::tag_add_arg;
use libimagutil::cli_validators::*;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("add")
                   .about("Add bookmarks")
                   .version("0.1")
                   .arg(Arg::with_name("collection")
                        .long("collection")
                        .short("c")
                        .takes_value(true)
                        .required(true)
                        .multiple(false)
                        .value_name("COLLECTION")
                        .help("Add to this collection"))
                   .arg(Arg::with_name("urls")
                        .long("urls")
                        .short("u")
                        .takes_value(true)
                        .required(true)
                        .multiple(true)
                        .value_name("URL")
                        .validator(is_url)
                        .help("Add this URL, multiple possible"))
                   .arg(tag_add_arg())
                   )

        .subcommand(SubCommand::with_name("remove")
                   .about("Remove bookmarks")
                   .version("0.1")
                   .arg(Arg::with_name("collection")
                        .long("collection")
                        .short("c")
                        .takes_value(true)
                        .required(true)
                        .multiple(false)
                        .value_name("COLLECTION")
                        .help("Remove from this collection"))
                   .arg(Arg::with_name("urls")
                        .long("urls")
                        .short("u")
                        .takes_value(true)
                        .required(true)
                        .multiple(true)
                        .value_name("URL")
                        .validator(is_url)
                        .help("Remove these urls, regex supported"))
                   )

        // .subcommand(SubCommand::with_name("open")
        //            .about("Open bookmarks (via xdg-open)")
        //            .version("0.1")
        //            .arg(Arg::with_name("collection")
        //                 .long("collection")
        //                 .short("c")
        //                 .takes_value(true)
        //                 .required(true)
        //                 .multiple(false)
        //                 .value_name("COLLECTION")
        //                 .help("Select from this collection"))
        //            )

        .subcommand(SubCommand::with_name("list")
                   .about("List bookmarks")
                   .version("0.1")
                   .arg(Arg::with_name("collection")
                        .long("collection")
                        .short("c")
                        .takes_value(true)
                        .required(true)
                        .multiple(false)
                        .value_name("COLLECTION")
                        .help("Select from this collection"))
                   .arg(Arg::with_name("tags")
                        .long("tags")
                        .short("t")
                        .takes_value(true)
                        .required(false)
                        .multiple(true)
                        .value_name("TAGS")
                        .help("Filter links to contain these tags. When multiple tags are specified, all of them must be set for the link to match."))
                   )

        .subcommand(SubCommand::with_name("collection")
                   .about("Collection commands")
                   .version("0.1")
                   .arg(Arg::with_name("add")
                        .long("add")
                        .short("a")
                        .takes_value(true)
                        .value_name("NAME")
                        .help("Add a collection with this name"))
                   .arg(Arg::with_name("remove")
                        .long("remove")
                        .short("r")
                        .takes_value(true)
                        .value_name("NAME")
                        .help("Remove a collection with this name (and all links)"))
                   )
}
