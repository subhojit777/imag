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

use std::io::stdout;
use std::io::Write;

use lister::Lister;
use error::Result;
use error::ResultExt;

use libimagstore::store::FileLockEntry;

pub struct LineLister<'a> {
    unknown_output: &'a str,
}

impl<'a> LineLister<'a> {

    pub fn new(unknown_output: &'a str) -> LineLister<'a> {
        LineLister {
            unknown_output: unknown_output,
        }
    }

}

impl<'a> Lister for LineLister<'a> {

    fn list<'b, I: Iterator<Item = FileLockEntry<'b>>>(&self, entries: I) -> Result<()> {
        use error::ListErrorKind as LEK;

        for entry in entries {
            let s = entry.get_location().to_str().unwrap_or(String::from(self.unknown_output));
            write!(stdout(), "{:?}\n", s).chain_err(|| LEK::FormatError)?
        }

        Ok(())
    }

}
