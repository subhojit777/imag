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

use libimagstore::store::FileLockEntry;
use libimagstore::store::Entry;

pub struct CoreLister<T: Fn(&Entry) -> String> {
    lister: Box<T>,
}

impl<T: Fn(&Entry) -> String> CoreLister<T> {

    pub fn new(lister: T) -> CoreLister<T> {
        CoreLister {
            lister: Box::new(lister),
        }
    }

}

impl<T: Fn(&Entry) -> String> Lister for CoreLister<T> {

    fn list<'b, I: Iterator<Item = FileLockEntry<'b>>>(&self, entries: I) -> Result<()> {
        use error::ListError as LE;
        use error::ListErrorKind as LEK;

        debug!("Called list()");
        let (r, n) = entries
            .fold((Ok(()), 0), |(accu, i), entry| {
                debug!("fold({:?}, {:?})", accu, entry);
                let r = accu.and_then(|_| {
                        debug!("Listing Entry: {:?}", entry);
                        write!(stdout(), "{:?}\n", (self.lister)(&entry))
                            .map_err(|e| LE::new(LEK::FormatError, Some(Box::new(e))))
                    });
                (r, i + 1)
            });
        debug!("Iterated over {} entries", n);
        r
    }

}

