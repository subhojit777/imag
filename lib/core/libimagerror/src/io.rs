//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 the imag contributors
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

use std::io::ErrorKind;

use exit::ExitCode;

pub enum Settings {
    Ignore(ErrorKind),
    IgnoreAny(Vec<ErrorKind>),
}

pub trait ToExitCode<T> {
    fn to_exit_code(self) -> Result<T, ExitCode>;
    fn to_exit_code_with(self, Settings) -> Result<T, ExitCode>;
}

impl<T> ToExitCode<T> for Result<T, ::std::io::Error> {

    /// Returns an exit code of 0 if the error was a broken pipe, else 1
    fn to_exit_code(self) -> Result<T, ExitCode> {
        self.to_exit_code_with(Settings::Ignore(ErrorKind::BrokenPipe))
    }

    /// Returns an exit code depending on the settings
    ///
    /// Via the settings, errors can be ignores (translates to exit code zero). All other errors
    /// are translated into exit code 1
    ///
    fn to_exit_code_with(self, settings: Settings) -> Result<T, ExitCode> {
        self.map_err(move |e| match settings {
            Settings::Ignore(kind) => if e.kind() == kind {
                0
            } else {
                1
            },
            Settings::IgnoreAny(v) => if v.iter().any(|el| e.kind() == *el) {
                0
            } else {
                1
            },
        })
        .map_err(ExitCode::from)
    }

}
