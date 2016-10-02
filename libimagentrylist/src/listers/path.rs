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
use result::Result;
use error::MapErrInto;

use libimagstore::store::FileLockEntry;
use libimagutil::iter::FoldResult;

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
        use error::ListError as LE;
        use error::ListErrorKind as LEK;

        entries.fold_defresult(|entry| {
            Ok(entry.get_location().clone())
                .and_then(|pb| pb.into_pathbuf().map_err_into(LEK::FormatError))
                .and_then(|pb| {
                    if self.absolute {
                        pb.canonicalize().map_err(|e| LE::new(LEK::FormatError, Some(Box::new(e))))
                    } else {
                        Ok(pb.into())
                    }
                })
                .and_then(|pb| {
                    write!(stdout(), "{:?}\n", pb)
                        .map_err(|e| LE::new(LEK::FormatError, Some(Box::new(e))))
                })
                .map_err(|e| {
                    if e.err_type() == LEK::FormatError {
                        e
                    } else {
                        LE::new(LEK::FormatError, Some(Box::new(e)))
                    }
                })
            })
    }

}

