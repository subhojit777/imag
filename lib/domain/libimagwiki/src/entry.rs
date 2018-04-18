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

use libimagstore::store::Store;
use libimagstore::store::Entry;
use libimagentrymarkdown::processor::LinkProcessor;

use error::Result;
use error::WikiErrorKind as WEK;
use error::ResultExt;

pub trait WikiEntry {
    fn autolink(&mut self, store: &Store) -> Result<()>;
    fn autolink_with_processor(&mut self, store: &Store, processor: LinkProcessor) -> Result<()>;
}

impl WikiEntry for Entry {

    /// Autolink entry to entries linked in content
    ///
    /// Uses `libimagentrymarkdown::processor::LinkProcessor` for this, with the following settings:
    ///
    /// * Interal link processing   = true
    /// * Internal targets creating = true
    /// * External link processing  = true
    /// * Processing of Refs        = true
    ///
    /// This is a convenience function for `WikiEntry::autolink_with_processor()`.
    ///
    /// # Warning
    ///
    /// With this function, the `LinkProcessor` automatically creates entries in the store if they
    /// are linked from the current entry but do not exists yet.
    ///
    /// # See also
    ///
    /// * The documentation of `WikiEntry::autolink_with_processor()`.
    /// * The documentation of `::libimagentrymarkdown::processor::LinkProcessor`.
    ///
    fn autolink(&mut self, store: &Store) -> Result<()> {
        let processor = LinkProcessor::default()
            .process_internal_links(true)
            .create_internal_targets(true)
            .process_external_links(true)
            .process_refs(true);

        self.autolink_with_processor(store, processor)
    }

    /// Autolink entry to entries linked in content with the passed `LinkProcessor` instance.
    ///
    /// See the documentation of `::libimagentrymarkdown::processor::LinkProcessor`.
    fn autolink_with_processor(&mut self, store: &Store, processor: LinkProcessor) -> Result<()> {
        processor.process(self, store).chain_err(|| WEK::AutoLinkError(self.get_location().clone()))
    }

}
