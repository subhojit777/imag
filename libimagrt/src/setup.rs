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

use clap::App;

use runtime::Runtime;

pub type Name          = &'static str;
pub type Version<'a>   = &'a str;
pub type About         = &'static str;

/// Helper to generate the Runtime object
///
/// exit()s the program if the runtime couldn't be build, prints error with println!() before
/// exiting
pub fn generate_runtime_setup<'a, B>(name: Name, version: Version<'a>, about: About, builder: B)
    -> Runtime<'a>
    where B: FnOnce(App<'a, 'a>) -> App<'a, 'a>
{
    use std::process::exit;
    use libimagerror::trace::trace_error_dbg;

    Runtime::new(builder(Runtime::get_default_cli_builder(name, version, about)))
        .unwrap_or_else(|e| {
            println!("Could not set up Runtime");
            println!("{:?}", e);
            trace_error_dbg(&e);
            exit(1);
        })
}
