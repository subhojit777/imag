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
    fn get_external_links(&self, store: &Store) -> Result<Vec<Url>>;

    /// Set the external links for the implementor object
    fn set_external_links(&mut self, store: &Store, links: Vec<Url>) -> Result<()>;

    /// Add an external link to the implementor object
    fn add_external_link(&mut self, store: &Store, link: Url) -> Result<()>;

    /// Remove an external link from the implementor object
    fn remove_external_link(&mut self, store: &Store, link: Url) -> Result<()>;

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
    fn get_external_links(&self, store: &Store) -> Result<Vec<Url>> {
        // Iterate through all internal links and filter for FileLockEntries which live in
        // /link/external/<SHA> -> load these files and get the external link from their headers,
        // put them into the return vector.
        self.get_internal_links()
            .map(|vect| {
                debug!("Getting external links");
                vect.into_iter()
                    .filter(is_external_link_storeid)
                    .map(|id| {
                        debug!("Retrieving entry for id: '{:?}'", id);
                        match store.retrieve(id.clone()) {
                            Ok(f) => {
                                debug!("Store::retrieve({:?}) succeeded", id);
                                debug!("getting external link from file now");
                                get_external_link_from_file(&f)
                                    .map_err(|e| { debug!("URL -> Err = {:?}", e); e })
                            },
                            Err(e) => {
                                debug!("Retrieving entry for id: '{:?}' failed", id);
                                Err(LE::new(LEK::StoreReadError, Some(Box::new(e))))
                            }
                        }
                    })
                    .filter_map(|x| x.ok()) // TODO: Do not ignore error here
                    .collect()
            })
            .map_err(|e| LE::new(LEK::StoreReadError, Some(Box::new(e))))
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
            .and_then(|mut links| {
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
                debug!("Removing link = '{:?}' from links = {:?}", link, links);
                let links = links.into_iter()
                    .filter(|l| l.as_str() != link.as_str())
                    .collect();
                self.set_external_links(store, links)
            })
    }

}

