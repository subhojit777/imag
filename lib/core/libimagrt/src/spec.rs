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

use std::io::Write;

use clap::{App, ArgMatches, Shell};

/// An abstraction over `clap::App` functionality needed for initializing `Runtime`. Different
/// implementations can be used for testing `imag` binaries without running them as separate
/// processes.
pub trait CliSpec<'a> {
    fn name(&self) -> &str;
    fn matches(self) -> ArgMatches<'a>;
    fn completions<W: Write, S: Into<String>>(&mut self, _: S, _: Shell, _: &mut W) {}
}

impl<'a> CliSpec<'a> for App<'a, 'a> {
    fn name(&self) -> &str {
        self.get_name()
    }

    fn matches(self) -> ArgMatches<'a> {
        self.get_matches()
    }

    fn completions<W: Write, S: Into<String>>(&mut self,
                                              bin_name: S,
                                              for_shell: Shell,
                                              buf: &mut W) {

        self.gen_completions_to(bin_name, for_shell, buf);
    }
}
