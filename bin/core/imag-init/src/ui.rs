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
        .arg(Arg::with_name("devel")
             .long("dev")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Put dev configuration into the generated repo (with debugging enabled)"))

        .arg(Arg::with_name("nogit")
             .long("no-git")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Do not initialize git repository, even if 'git' executable is in $PATH"))

        .arg(Arg::with_name("path")
             .long("path")
             .takes_value(true)
             .required(false)
             .multiple(false)
             .help("Alternative path where to put the repository. Default: ~/.imag"))
}

