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

use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagutil::debug_result::*;

use error::LinkError as LE;
use error::LinkErrorKind as LEK;
use error::MapErrInto;
use result::Result;
use internal::InternalLinker;
use module_path::ModuleEntryPath;

use self::iter::*;

use toml::Value;
use url::Url;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

/// "Link" Type, just an abstraction over `FileLockEntry` to have some convenience internally.
pub struct Link<'a> {
    link: FileLockEntry<'a>
}

impl<'a> Link<'a> {

    pub fn new(fle: FileLockEntry<'a>) -> Link<'a> {
        Link { link: fle }
    }

    /// Get a link Url object from a `FileLockEntry`, ignore errors.
    fn get_link_uri_from_filelockentry(file: &FileLockEntry<'a>) -> Option<Url> {
        file.get_header()
            .read("imag.content.url")
            .ok()
            .and_then(|opt| match opt {
                Some(Value::String(s)) => {
                    debug!("Found url, parsing: {:?}", s);
                    Url::parse(&s[..]).ok()
                },
                _ => None
            })
    }

    pub fn get_url(&self) -> Result<Option<Url>> {
        let opt = self.link
            .get_header()
            .read("imag.content.url");

        match opt {
            Ok(Some(Value::String(s))) => {
                Url::parse(&s[..])
                     .map(Some)
                     .map_err(|e| LE::new(LEK::EntryHeaderReadError, Some(Box::new(e))))
            },
            Ok(None) => Ok(None),
            _ => Err(LE::new(LEK::EntryHeaderReadError, None))
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
    use error::LinkErrorKind as LEK;
    use error::MapErrInto;
    use result::Result;

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
    ///        1 |    1 |   1 |   0
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
                if !(self.1 ^ is_external_link_storeid(&elem)) {
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
            use super::get_external_link_from_file;

            self.0
                .next()
                .map(|id| {
                    debug!("Retrieving entry for id: '{:?}'", id);
                    self.1
                        .retrieve(id.clone())
                        .map_err_into(LEK::StoreReadError)
                        .map_dbg_err(|_| format!("Retrieving entry for id: '{:?}' failed", id))
                        .and_then(|f| {
                            debug!("Store::retrieve({:?}) succeeded", id);
                            debug!("getting external link from file now");
                            get_external_link_from_file(&f)
                                .map_dbg_err(|e| format!("URL -> Err = {:?}", e))
                        })
                })
        }

    }

}


/// Check whether the StoreId starts with `/link/external/`
pub fn is_external_link_storeid(id: &StoreId) -> bool {
    debug!("Checking whether this is a 'links/external/': '{:?}'", id);
    id.local().starts_with("links/external")
}

fn get_external_link_from_file(entry: &FileLockEntry) -> Result<Url> {
    Link::get_link_uri_from_filelockentry(entry) // TODO: Do not hide error by using this function
        .ok_or(LE::new(LEK::StoreReadError, None))
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
            .map_err(|e| LE::new(LEK::StoreReadError, Some(Box::new(e))))
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
            let file_id = try!(
                ModuleEntryPath::new(format!("external/{}", hash)).into_storeid()
                    .map_err_into(LEK::StoreWriteError)
                    .map_dbg_err(|_| {
                        format!("Failed to build StoreId for this hash '{:?}'", hash)
                    })
                );

            debug!("Link    = '{:?}'", link);
            debug!("Hash    = '{:?}'", hash);
            debug!("StoreId = '{:?}'", file_id);

            // retrieve the file from the store, which implicitely creates the entry if it does not
            // exist
            let mut file = try!(store
                .retrieve(file_id.clone())
                .map_err_into(LEK::StoreWriteError)
                .map_dbg_err(|_| {
                    format!("Failed to create or retrieve an file for this link '{:?}'", link)
                }));

            debug!("Generating header content!");
            {
                let mut hdr = file.deref_mut().get_header_mut();

                let mut table = match hdr.read("imag.content") {
                    Ok(Some(Value::Table(table))) => table,
                    Ok(Some(_)) => {
                        warn!("There is a value at 'imag.content' which is not a table.");
                        warn!("Going to override this value");
                        BTreeMap::new()
                    },
                    Ok(None) => BTreeMap::new(),
                    Err(e)   => return Err(LE::new(LEK::StoreWriteError, Some(Box::new(e)))),
                };

                let v = Value::String(link.into_string());

                debug!("setting URL = '{:?}", v);
                table.insert(String::from("url"), v);

                if let Err(e) = hdr.set("imag.content", Value::Table(table)) {
                    return Err(LE::new(LEK::StoreWriteError, Some(Box::new(e))));
                } else {
                    debug!("Setting URL worked");
                }
            }

            // then add an internal link to the new file or return an error if this fails
            if let Err(e) = self.add_internal_link(file.deref_mut()) {
                debug!("Error adding internal link");
                return Err(LE::new(LEK::StoreWriteError, Some(Box::new(e))));
            }
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

