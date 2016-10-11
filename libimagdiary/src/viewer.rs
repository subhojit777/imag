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

use entry::Entry;
use error::DiaryErrorKind as DEK;
use error::MapErrInto;
use result::Result;

use libimagentryview::viewer::Viewer;
use libimagentryview::builtin::plain::PlainViewer;

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
    pub fn view_entries<'a, I: Iterator<Item = Entry<'a>>>(&self, entries: I) -> Result<()> {
        for entry in entries {
            let id = entry.diary_id();
            println!("{} :\n", id);
            let _ = try!(self.0
                         .view_entry(&entry)
                         .map_err_into(DEK::ViewError)
                         .map_err_into(DEK::IOError));
            println!("\n---\n");
        }

        Ok(())
    }

}

