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

pub struct ExitCode(i32);

impl From<i32> for ExitCode {
    fn from(i: i32) -> ExitCode {
        ExitCode(i)
    }
}

pub trait ExitUnwrap<T> {
    fn unwrap_or_exit(self) -> T;
}

impl<T, E: Into<ExitCode>> ExitUnwrap<T> for Result<T, E> {
    fn unwrap_or_exit(self) -> T {
        self.map_err(Into::into).unwrap_or_else(|e| ::std::process::exit(e.0))
    }
}

