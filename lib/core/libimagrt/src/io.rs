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

//! Proxy objects for std::io::Stdin, std::io::Stdout, std::io::Stderr

use std::fmt::Debug;
use std::io::Write;

/// Proxy object for output
///
/// This is returned by `Runtime::stdout()` does implement `Write`. So you can
/// `write!(rt.stdout(), "some things")` and it just works.
///
/// The `Runtime` has to decide whether the OutputProxy should write to stdout, stderr or simply be
/// a "sink" which does not write to either.
///
pub enum OutputProxy {
    Out,
    Err,
    Sink,
}

impl Write for OutputProxy {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        match *self {
            OutputProxy::Out  => ::std::io::stdout().write(buf),
            OutputProxy::Err  => ::std::io::stderr().write(buf),
            OutputProxy::Sink => Ok(0),
        }
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        match *self {
            OutputProxy::Out  => ::std::io::stdout().flush(),
            OutputProxy::Err  => ::std::io::stderr().flush(),
            OutputProxy::Sink => Ok(()),
        }
    }

}

impl Debug for OutputProxy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            OutputProxy::Out  => write!(f, "OutputProxy(Stdout)"),
            OutputProxy::Err  => write!(f, "OutputProxy(Stderr)"),
            OutputProxy::Sink => write!(f, "OutputProxy(Sink)"),
        }
    }
}
