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

use std::io::stdout;
use std::io::Write;

use lister::Lister;
use error::Result;
use error::ResultExt;

use libimagstore::store::FileLockEntry;

pub struct PathLister {
    absolute: bool,
}

impl PathLister {

    pub fn new(absolute: bool) -> PathLister {
        PathLister {
            absolute: absolute,
        }
    }

}

impl Lister for PathLister {

    fn list<'a, I: Iterator<Item = FileLockEntry<'a>>>(&self, entries: I) -> Result<()> {
        use error::ListErrorKind as LEK;

        for entry in entries {
            let pb = entry.get_location().clone();
            let pb = pb.into_pathbuf().chain_err(|| LEK::FormatError)?;
            let pb = if self.absolute {
                pb.canonicalize().chain_err(|| LEK::FormatError)?
            } else {
                pb.into()
            };

            write!(stdout(), "{:?}\n", pb).chain_err(|| LEK::FormatError)?
        }

        Ok(())
    }

}

