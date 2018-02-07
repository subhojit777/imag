//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use clap::{Arg, App};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .arg(Arg::with_name("files-with-matches")
             .long("files-with-matches")
             .short("l")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("List files with matches"))

        .arg(Arg::with_name("count")
             .long("count")
             .short("c")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Count matches"))

        .arg(Arg::with_name("pattern")
             .index(1)
             .takes_value(false)
             .required(true)
             .multiple(false)
             .value_name("PATTERN")
             .help("Pattern to search for. Regex is supported, multiple patterns are not."))
}
