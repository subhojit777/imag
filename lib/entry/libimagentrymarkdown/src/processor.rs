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

use error::MarkdownError as ME;
use error::MarkdownErrorKind as MEK;
use error::*;
use link::extract_links;

use libimagentrylink::external::ExternalLinker;
use libimagentrylink::internal::InternalLinker;
use libimagentryref::refstore::RefStore;
use libimagentryref::flags::RefFlags;
use libimagstore::store::Entry;
use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimagutil::iter::FoldResult;

use std::path::PathBuf;

use url::Url;

/// A link Processor which collects the links from a Markdown and passes them on to
/// `libimagentrylink` functionality
///
/// The processor can be configured to
///
///  * Process internal links (from store entry to store entry)
///  * Process internal links with automatically creating targets
///     If an internal link is encountered, the corrosponding target must be present in the store.
///     If it is not, it will either be created or the processing fails
///  * Process external links (from store entry to URL)
///  * Process refs (from store entry to files on the filesystem and outside of the store)
///  (default: false)
///
///  # Note
///
///  There's no LinkProcessor::new() function, please use `LinkProcessor::default()`.
///
pub struct LinkProcessor {
    process_internal_links: bool,
    create_internal_targets: bool,
    process_external_links: bool,
    process_refs: bool
}

impl LinkProcessor {

    /// Switch internal link processing on/off
    ///
    /// Internal links are links which are simply `dirctory/file`, but not `/directory/file`, as
    /// beginning an id with `/` is discouraged in imag.
    pub fn process_internal_links(mut self, b: bool) -> Self {
        self.process_internal_links = b;
        self
    }

    /// Switch internal link target creation on/off
    ///
    /// If a link points to a non-existing imag entry, a `false` here will cause the processor to
    /// return an error from `process()`. A `true` setting will create the entry and then fetch it
    /// to link it to the processed entry.
    pub fn create_internal_targets(mut self, b: bool) -> Self {
        self.create_internal_targets = b;
        self
    }

    /// Switch external link processing on/off
    ///
    /// An external link must start with `https://` or `http://`.
    pub fn process_external_links(mut self, b: bool) -> Self {
        self.process_external_links = b;
        self
    }

    /// Switch ref processing on/off
    ///
    /// A Ref is to be expected beeing a link with `file::///` at the beginning.
    pub fn process_refs(mut self, b: bool) -> Self {
        self.process_refs = b;
        self
    }

    /// Process an Entry for its links
    ///
    /// # Warning
    ///
    /// When `LinkProcessor::create_internal_targets()` was called to set the setting to true, this
    /// function returns all errors returned by the Store.
    ///
    pub fn process<'a>(&self, entry: &mut Entry, store: &'a Store) -> Result<()> {
        let text = entry.to_str();
        trace!("Processing: {:?}", entry.get_location());
        extract_links(&text)
            .into_iter()
            .fold_result::<_, MarkdownError, _>(|link| {
                trace!("Processing {:?}", link);
                match LinkQualification::qualify(&link.link) {
                    LinkQualification::InternalLink => {
                        if !self.process_internal_links {
                            return Ok(());
                        }

                        let spath      = Some(store.path().clone());
                        let id         = StoreId::new(spath, PathBuf::from(&link.link))?;
                        let mut target = if self.create_internal_targets {
                            try!(store.retrieve(id))
                        } else {
                            store.get(id.clone())?
                                .ok_or(ME::from_kind(MEK::StoreGetError(id)))?
                        };

                        entry.add_internal_link(&mut target).map_err(From::from)
                    },
                    LinkQualification::ExternalLink(url) => {
                        if !self.process_external_links {
                            return Ok(());
                        }

                        entry.add_external_link(store, url).map_err(From::from)
                    },
                    LinkQualification::RefLink(url) => {
                        if !self.process_refs {
                            return Ok(());
                        }

                        let flags = RefFlags::default()
                            .with_content_hashing(false)
                            .with_permission_tracking(false);
                        trace!("URL            = {:?}", url);
                        trace!("URL.path()     = {:?}", url.path());
                        trace!("URL.host_str() = {:?}", url.host_str());
                        let path = url.host_str().unwrap_or_else(|| url.path());
                        let path = PathBuf::from(path);
                        let mut target = try!(RefStore::create(store, path, flags));

                        entry.add_internal_link(&mut target).map_err(From::from)
                    },
                    LinkQualification::Undecidable(e) => {
                        // error
                        Err(e).chain_err(|| MEK::UndecidableLinkType(link.link.clone()))
                    },
                }
            })
    }

}

/// Enum to tell what kind of link a string of text is
enum LinkQualification {
    InternalLink,
    ExternalLink(Url),
    RefLink(Url),
    Undecidable(ME),
}

impl LinkQualification {
    fn qualify(text: &str) -> LinkQualification {
        match Url::parse(text) {
            Ok(url) => {
                if url.scheme() == "file" {
                    return LinkQualification::RefLink(url)
                }

                // else we assume the following, as other stuff gets thrown out by
                // url::Url::parse() as Err(_)
                //
                // if url.scheme() == "https" || url.scheme() == "http" {
                    return LinkQualification::ExternalLink(url);
                // }
            },

            Err(e) => {
                match e {
                    ::url::ParseError::RelativeUrlWithoutBase => {
                        LinkQualification::InternalLink
                    },

                    _ => LinkQualification::Undecidable(ME::from(e)),
                }
            }
        }
    }
}

impl Default for LinkProcessor {
    fn default() -> Self {
        LinkProcessor {
            process_internal_links: true,
            create_internal_targets: false,
            process_external_links: true,
            process_refs: false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use libimagstore::store::Store;
    use libimagentrylink::internal::InternalLinker;

    fn setup_logging() {
        use env_logger;
        let _ = env_logger::init().unwrap_or(());
    }

    pub fn get_store() -> Store {
        use libimagstore::file_abstraction::InMemoryFileAbstraction;
        let fs = InMemoryFileAbstraction::new();
        Store::new_with_backend(PathBuf::from("/"), None, Box::new(fs)).unwrap()
    }

    #[test]
    fn test_process_no_links() {
        setup_logging();
        let store = get_store();

        let mut base = store.create(PathBuf::from("test-1")).unwrap();
        *base.get_content_mut() = format!("This is an example entry with no links");

        let update = store.update(&mut base);
        assert!(update.is_ok());

        let processor = LinkProcessor::default();

        let result = processor.process(&mut base, &store);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_one_existing_file_linked() {
        setup_logging();
        let store = get_store();

        let mut base = store.create(PathBuf::from("test-2.1")).unwrap();
        *base.get_content_mut() = format!("This is an example entry with one [link](test-2.2)");

        let update = store.update(&mut base);
        assert!(update.is_ok());

        // immediately drop as we don't need this entry right now
        let _ = store.create(PathBuf::from("test-2.2")).unwrap();

        let processor = LinkProcessor::default()
            .process_internal_links(true)
            .create_internal_targets(false)
            .process_external_links(false)
            .process_refs(false);

        let result = processor.process(&mut base, &store);
        assert!(result.is_ok(), "Should be Ok(()): {:?}", result);

        {
            let base_links = base.get_internal_links();
            assert!(base_links.is_ok());
            let base_links : Vec<_> = base_links.unwrap().collect();

            assert_eq!(1, base_links.len());
            assert_eq!("test-2.2", base_links[0].to_str().unwrap());
        }

        {
            let link = store.get(PathBuf::from("test-2.2")).unwrap().unwrap();
            let link_links = link.get_internal_links();
            assert!(link_links.is_ok());
            let link_links : Vec<_> = link_links.unwrap().collect();

            assert_eq!(1, link_links.len());
            assert_eq!("test-2.1", link_links[0].to_str().unwrap());
        }
    }

    #[test]
    fn test_process_one_existing_file_linked_faulty() {
        setup_logging();
        let store = get_store();

        let mut base = store.create(PathBuf::from("test-2.1")).unwrap();
        *base.get_content_mut() = format!("This is an example entry with one [link](/test-2.2)");

        let update = store.update(&mut base);
        assert!(update.is_ok());

        let processor = LinkProcessor::default()
            .process_internal_links(true)
            .create_internal_targets(false)
            .process_external_links(false)
            .process_refs(false);

        let result = processor.process(&mut base, &store);
        assert!(result.is_err(), "Should be Err(_), but is Ok(())");
    }

    #[test]
    fn test_process_one_nonexisting_file_linked() {
        setup_logging();
        let store = get_store();

        let mut base = store.create(PathBuf::from("test-2.1")).unwrap();
        *base.get_content_mut() = format!("This is an example entry with one [link](test-2.2)");

        let update = store.update(&mut base);
        assert!(update.is_ok());

        let processor = LinkProcessor::default()
            .process_internal_links(true)
            .create_internal_targets(true)
            .process_external_links(false)
            .process_refs(false);

        let result = processor.process(&mut base, &store);
        assert!(result.is_ok(), "Should be Ok(()): {:?}", result);

        {
            let base_links = base.get_internal_links();
            assert!(base_links.is_ok());
            let base_links : Vec<_> = base_links.unwrap().collect();

            assert_eq!(1, base_links.len());
            assert_eq!("test-2.2", base_links[0].to_str().unwrap());
        }

        {
            let link = store.get(PathBuf::from("test-2.2")).unwrap().unwrap();
            let link_links = link.get_internal_links();
            assert!(link_links.is_ok());
            let link_links : Vec<_> = link_links.unwrap().collect();

            assert_eq!(1, link_links.len());
            assert_eq!("test-2.1", link_links[0].to_str().unwrap());
        }
    }
}

