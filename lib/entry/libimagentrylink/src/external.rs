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

/// External linking is a complex implementation to be able to serve a clean and easy-to-use
/// interface.
///
/// Internally, there are no such things as "external links" (plural). Each Entry in the store can
/// only have _one_ external link.
///
/// This library does the following therefor: It allows you to have several external links with one
/// entry, which are internally one file in the store for each link, linked with "internal
/// linking".
///
/// This helps us greatly with deduplication of URLs.
///

use std::ops::DerefMut;
use std::collections::BTreeMap;
use std::fmt::Debug;

use libimagstore::store::Entry;
use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagutil::debug_result::*;

use toml_query::read::TomlValueReadExt;
use toml_query::read::TomlValueReadTypeExt;
use toml_query::insert::TomlValueInsertExt;

use error::LinkErrorKind as LEK;
use error::Result;
use internal::InternalLinker;
use module_path::ModuleEntryPath;
use error::ResultExt;

use self::iter::*;

use toml::Value;
use url::Url;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

pub trait Link {

    fn get_link_uri_from_filelockentry(&self) -> Result<Option<Url>>;

    fn get_url(&self) -> Result<Option<Url>>;

}

impl Link for Entry {

    fn get_link_uri_from_filelockentry(&self) -> Result<Option<Url>> {
        self.get_header()
            .read_string("links.external.content.url")
            .chain_err(|| LEK::EntryHeaderReadError)
            .and_then(|opt| match opt {
                None        => Ok(None),
                Some(ref s) => {
                    debug!("Found url, parsing: {:?}", s);
                    Url::parse(&s[..]).chain_err(|| LEK::InvalidUri).map(Some)
                },
            })
    }

    fn get_url(&self) -> Result<Option<Url>> {
        match self.get_header().read_string("links.external.url")? {
            None        => Ok(None),
            Some(ref s) => Url::parse(&s[..])
                .map(Some)
                .chain_err(|| LEK::EntryHeaderReadError),
        }
    }

}

pub trait ExternalLinker : InternalLinker {

    /// Get the external links from the implementor object
    fn get_external_links<'a>(&self, store: &'a Store) -> Result<UrlIter<'a>>;

    /// Set the external links for the implementor object
    fn set_external_links(&mut self, store: &Store, links: Vec<Url>) -> Result<()>;

    /// Add an external link to the implementor object
    fn add_external_link(&mut self, store: &Store, link: Url) -> Result<()>;

    /// Remove an external link from the implementor object
    fn remove_external_link(&mut self, store: &Store, link: Url) -> Result<()>;

}

pub mod iter {
    //! Iterator helpers for external linking stuff
    //!
    //! Contains also helpers to filter iterators for external/internal links
    //!
    //!
    //! # Warning
    //!
    //! This module uses `internal::Link` as link type, so we operate on _store ids_ here.
    //!
    //! Not to confuse with `external::Link` which is a real `FileLockEntry` under the hood.
    //!

    use libimagutil::debug_result::*;
    use libimagstore::store::Store;

    use internal::Link;
    use internal::iter::LinkIter;
    use error::Result;

    use url::Url;

    /// Helper for building `OnlyExternalIter` and `NoExternalIter`
    ///
    /// The boolean value defines, how to interpret the `is_external_link_storeid()` return value
    /// (here as "pred"):
    ///
    ///     pred | bool | xor | take?
    ///     ---- | ---- | --- | ----
    ///        0 |    0 |   0 |   1
    ///        0 |    1 |   1 |   0
    ///        1 |    0 |   1 |   0
    ///        1 |    1 |   0 |   1
    ///
    /// If `bool` says "take if return value is false", we take the element if the `pred` returns
    /// false... and so on.
    ///
    /// As we can see, the operator between these two operants is `!(a ^ b)`.
    pub struct ExternalFilterIter(LinkIter, bool);

    impl Iterator for ExternalFilterIter {
        type Item = Link;

        fn next(&mut self) -> Option<Self::Item> {
            use super::is_external_link_storeid;

            while let Some(elem) = self.0.next() {
                trace!("Check whether is external: {:?}", elem);
                if !(self.1 ^ is_external_link_storeid(&elem)) {
                    trace!("Is external id: {:?}", elem);
                    return Some(elem);
                }
            }
            None
        }
    }

    /// Helper trait to be implemented on `LinkIter` to select or deselect all external links
    ///
    /// # See also
    ///
    /// Also see `OnlyExternalIter` and `NoExternalIter` and the helper traits/functions
    /// `OnlyInteralLinks`/`only_internal_links()` and `OnlyExternalLinks`/`only_external_links()`.
    pub trait SelectExternal {
        fn select_external_links(self, b: bool) -> ExternalFilterIter;
    }

    impl SelectExternal for LinkIter {
        fn select_external_links(self, b: bool) -> ExternalFilterIter {
            ExternalFilterIter(self, b)
        }
    }


    pub struct OnlyExternalIter(ExternalFilterIter);

    impl OnlyExternalIter {
        pub fn new(li: LinkIter) -> OnlyExternalIter {
            OnlyExternalIter(ExternalFilterIter(li, true))
        }

        pub fn urls<'a>(self, store: &'a Store) -> UrlIter<'a> {
            UrlIter(self, store)
        }
    }

    impl Iterator for OnlyExternalIter {
        type Item = Link;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    pub struct NoExternalIter(ExternalFilterIter);

    impl NoExternalIter {
        pub fn new(li: LinkIter) -> NoExternalIter {
            NoExternalIter(ExternalFilterIter(li, false))
        }
    }

    impl Iterator for NoExternalIter {
        type Item = Link;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    pub trait OnlyExternalLinks : Sized {
        fn only_external_links(self) -> OnlyExternalIter ;

        fn no_internal_links(self) -> OnlyExternalIter {
            self.only_external_links()
        }
    }

    impl OnlyExternalLinks for LinkIter {
        fn only_external_links(self) -> OnlyExternalIter {
            OnlyExternalIter::new(self)
        }
    }

    pub trait OnlyInternalLinks : Sized {
        fn only_internal_links(self) -> NoExternalIter;

        fn no_external_links(self) -> NoExternalIter {
            self.only_internal_links()
        }
    }

    impl OnlyInternalLinks for LinkIter {
        fn only_internal_links(self) -> NoExternalIter {
            NoExternalIter::new(self)
        }
    }

    pub struct UrlIter<'a>(OnlyExternalIter, &'a Store);

    impl<'a> Iterator for UrlIter<'a> {
        type Item = Result<Url>;

        fn next(&mut self) -> Option<Self::Item> {
            use external::Link;

            loop {
                let next = self.0
                    .next()
                    .map(|id| {
                        debug!("Retrieving entry for id: '{:?}'", id);
                        self.1
                            .retrieve(id.clone())
                            .map_dbg_err(|_| format!("Retrieving entry for id: '{:?}' failed", id))
                            .map_err(From::from)
                            .and_then(|f| {
                                debug!("Store::retrieve({:?}) succeeded", id);
                                debug!("getting external link from file now");
                                f.get_link_uri_from_filelockentry()
                                    .map_dbg_str("Error happened while getting link URI from FLE")
                                    .map_dbg_err(|e| format!("URL -> Err = {:?}", e))
                            })
                    });

                match next {
                    Some(Ok(Some(link))) => return Some(Ok(link)),
                    Some(Ok(None))       => continue,
                    Some(Err(e))         => return Some(Err(e)),
                    None                 => return None
                }
            }
        }

    }

}


/// Check whether the StoreId starts with `/link/external/`
pub fn is_external_link_storeid<A: AsRef<StoreId> + Debug>(id: A) -> bool {
    debug!("Checking whether this is a 'links/external/': '{:?}'", id);
    id.as_ref().local().starts_with("links/external")
}

/// Implement `ExternalLinker` for `Entry`, hiding the fact that there is no such thing as an external
/// link in an entry, but internal links to other entries which serve as external links, as one
/// entry in the store can only have one external link.
impl ExternalLinker for Entry {

    /// Get the external links from the implementor object
    fn get_external_links<'a>(&self, store: &'a Store) -> Result<UrlIter<'a>> {
        // Iterate through all internal links and filter for FileLockEntries which live in
        // /link/external/<SHA> -> load these files and get the external link from their headers,
        // put them into the return vector.
        self.get_internal_links()
            .map(|iter| {
                debug!("Getting external links");
                iter.only_external_links().urls(store)
            })
    }

    /// Set the external links for the implementor object
    fn set_external_links(&mut self, store: &Store, links: Vec<Url>) -> Result<()> {
        // Take all the links, generate a SHA sum out of each one, filter out the already existing
        // store entries and store the other URIs in the header of one FileLockEntry each, in
        // the path /link/external/<SHA of the URL>

        debug!("Iterating {} links = {:?}", links.len(), links);
        for link in links { // for all links
            let hash = {
                let mut s = Sha1::new();
                s.input_str(&link.as_str()[..]);
                s.result_str()
            };
            let file_id =
                ModuleEntryPath::new(format!("external/{}", hash)).into_storeid()
                    .map_dbg_err(|_| {
                        format!("Failed to build StoreId for this hash '{:?}'", hash)
                    })
                ?;

            debug!("Link    = '{:?}'", link);
            debug!("Hash    = '{:?}'", hash);
            debug!("StoreId = '{:?}'", file_id);

            // retrieve the file from the store, which implicitely creates the entry if it does not
            // exist
            let mut file = store
                .retrieve(file_id.clone())
                .map_dbg_err(|_| {
                    format!("Failed to create or retrieve an file for this link '{:?}'", link)
                })?;

            debug!("Generating header content!");
            {
                let hdr = file.deref_mut().get_header_mut();

                let mut table = match hdr.read("links.external.content")? {
                    Some(&Value::Table(ref table)) => table.clone(),
                    Some(_) => {
                        warn!("There is a value at 'links.external.content' which is not a table.");
                        warn!("Going to override this value");
                        BTreeMap::new()
                    },
                    None => BTreeMap::new(),
                };

                let v = Value::String(link.into_string());

                debug!("setting URL = '{:?}", v);
                table.insert(String::from("url"), v);

                let _ = hdr.insert("links.external.content", Value::Table(table))?;
                debug!("Setting URL worked");
            }

            // then add an internal link to the new file or return an error if this fails
            let _ = self.add_internal_link(file.deref_mut())?;
            debug!("Error adding internal link");
        }
        debug!("Ready iterating");
        Ok(())
    }

    /// Add an external link to the implementor object
    fn add_external_link(&mut self, store: &Store, link: Url) -> Result<()> {
        // get external links, add this one, save them
        debug!("Getting links");
        self.get_external_links(store)
            .and_then(|links| {
                // TODO: Do not ignore errors here
                let mut links = links.filter_map(Result::ok).collect::<Vec<_>>();
                debug!("Adding link = '{:?}' to links = {:?}", link, links);
                links.push(link);
                debug!("Setting {} links = {:?}", links.len(), links);
                self.set_external_links(store, links)
            })
    }

    /// Remove an external link from the implementor object
    fn remove_external_link(&mut self, store: &Store, link: Url) -> Result<()> {
        // get external links, remove this one, save them
        self.get_external_links(store)
            .and_then(|links| {
                debug!("Removing link = '{:?}'", link);
                let links = links
                    .filter_map(Result::ok)
                    .filter(|l| l.as_str() != link.as_str())
                    .collect::<Vec<_>>();
                self.set_external_links(store, links)
            })
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use libimagstore::store::Store;

    fn setup_logging() {
        use env_logger;
        let _ = env_logger::try_init();
    }

    pub fn get_store() -> Store {
        use libimagstore::file_abstraction::InMemoryFileAbstraction;
        let backend = Box::new(InMemoryFileAbstraction::default());
        Store::new_with_backend(PathBuf::from("/"), &None, backend).unwrap()
    }


    #[test]
    fn test_simple() {
        setup_logging();
        let store = get_store();
        let mut e = store.retrieve(PathBuf::from("base-test_simple")).unwrap();
        let url   = Url::parse("http://google.de").unwrap();

        assert!(e.add_external_link(&store, url.clone()).is_ok());

        assert_eq!(1, e.get_external_links(&store).unwrap().count());
        assert_eq!(url, e.get_external_links(&store).unwrap().next().unwrap().unwrap());
    }

}

