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

//! A diary viewer built on libimagentryview.

use entry::DiaryEntry;
use error::DiaryErrorKind as DEK;
use error::ResultExt;
use error::Result;

use libimagstore::store::FileLockEntry;
use libimagentryview::viewer::Viewer;
use libimagentryview::builtin::plain::PlainViewer;
use libimagerror::trace::trace_error;

/// This viewer does _not_ implement libimagentryview::viewer::Viewer because we need to be able to
/// call some diary-type specific functions on the entries passed to this.
///
/// This type is mainly just written to be constructed-called-deleted in one go:
///
/// ```ignore
/// DiaryViewer::new(show_header).view_entries(entries);
/// ```
///
pub struct DiaryViewer(PlainViewer);

impl DiaryViewer {

    pub fn new(show_header: bool) -> DiaryViewer {
        DiaryViewer(PlainViewer::new(show_header))
    }

    /// View all entries from the iterator, or stop immediately if an error occurs, returning that
    /// error.
    pub fn view_entries<'a, I: Iterator<Item = FileLockEntry<'a>>>(&self, entries: I) -> Result<()> {
        for entry in entries {
            match entry.diary_id() {
                Ok(id) => println!("{} :\n", id),
                Err(e) => trace_error(&e),
            }
            let _ = try!(self.0
                         .view_entry(&entry)
                         .chain_err(|| DEK::ViewError)
                         .chain_err(|| DEK::IOError));
            println!("\n---\n");
        }

        Ok(())
    }

}

