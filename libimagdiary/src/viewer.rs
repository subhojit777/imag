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

