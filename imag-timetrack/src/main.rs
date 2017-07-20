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

#[macro_use]
extern crate log;

#[macro_use]
extern crate version;

extern crate clap;
extern crate semver;
extern crate toml;
extern crate chrono;
extern crate filters;
extern crate itertools;

extern crate libimagerror;
extern crate libimagstore;
extern crate libimagrt;
extern crate libimagentrytimetrack;
extern crate libimagutil;

mod cont;
mod day;
mod month;
mod start;
mod stop;
mod track;
mod ui;
mod week;
mod year;

use cont::cont;
use day::day;
use month::month;
use start::start;
use stop::stop;
use track::track;
use ui::build_ui;
use week::week;
use year::year;

use libimagrt::setup::generate_runtime_setup;

fn main() {
    let rt = generate_runtime_setup("imag-timetrack",
                                    &version!()[..],
                                    "Time tracking module",
                                    build_ui);

    let command = rt.cli().subcommand_name();
    let retval  = if let Some(command) = command {
        debug!("Call: {}", command);
        match command {
            "continue" => cont(&rt),
            "day"      => day(&rt),
            "month"    => month(&rt),
            "start"    => start(&rt),
            "stop"     => stop(&rt),
            "track"    => track(&rt),
            "week"     => week(&rt),
            "year"     => year(&rt),
            _ => {
                error!("Unknown command");
                1
            },
        }
    } else {
        error!("No command");
        1
    };

    ::std::process::exit(retval);
}
