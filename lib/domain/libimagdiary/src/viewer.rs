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

//! A diary viewer built on libimagentryview.

use std::io::Write;
use std::ops::Deref;

use libimagstore::store::Entry;
use libimagentryview::viewer::Viewer;
use libimagentryview::error::ViewErrorKind as VEK;
use libimagentryview::error::ResultExt;
use libimagentryview::error::Result as ViewResult;
use libimagentryview::builtin::plain::PlainViewer;
use entry::DiaryEntry;

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

}

impl Viewer for DiaryViewer {

    fn view_entry<W>(&self, e: &Entry, sink: &mut W) -> ViewResult<()>
        where W: Write
    {
        self.0.view_entry(e, sink)
    }

    /// View all entries from the iterator, or stop immediately if an error occurs, returning that
    /// error.
    fn view_entries<I, E, W>(&self, entries: I, sink: &mut W) -> ViewResult<()>
        where I: Iterator<Item = E>,
              E: Deref<Target = Entry>,
              W: Write
    {
        let mut entries = entries
            .map(|e| e.deref().diary_id().map(|id| (id, e)).chain_err(|| VEK::ViewError))
            .collect::<ViewResult<Vec<_>>>()?;

        entries.sort_by_key(|&(ref id, _)| {
            [id.year() as u32, id.month(), id.day(), id.hour(), id.minute(), id.second()]
        });

        for (id, entry) in entries.into_iter() {
            writeln!(sink, "{} :\n", id)?;
            let _ = self.0.view_entry(entry.deref(), sink)?;
            writeln!(sink, "\n---\n")?;
        }

        Ok(())
    }

}

