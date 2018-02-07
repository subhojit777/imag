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

//! BookmarkCollection module
//!
//! A BookmarkCollection is nothing more than a simple store entry. One can simply call functions
//! from the libimagentrylink::external::ExternalLinker trait on this to generate external links.
//!
//! The BookmarkCollection type offers helper functions to get all links or such things.

use regex::Regex;

use error::Result;
use module_path::ModuleEntryPath;

use libimagstore::store::Store;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::IntoStoreId;
use libimagentrylink::external::ExternalLinker;
use libimagentrylink::external::iter::UrlIter;
use libimagentrylink::internal::InternalLinker;
use libimagentrylink::internal::Link as StoreLink;

use link::Link;

use self::iter::LinksMatchingRegexIter;

pub trait BookmarkCollectionStore<'a> {
    fn new(&'a self, name: &str)                     -> Result<FileLockEntry<'a>>;
    fn get(&'a self, name: &str)                     -> Result<Option<FileLockEntry<'a>>>;
    fn delete(&'a self, name: &str)                     -> Result<()>;
}

impl<'a> BookmarkCollectionStore<'a> for Store {

    fn new(&'a self, name: &str) -> Result<FileLockEntry<'a>> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| self.create(id).map_err(From::from))
            .map_err(From::from)
    }

    fn get(&'a self, name: &str) -> Result<Option<FileLockEntry<'a>>> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| self.get(id).map_err(From::from))
            .map_err(From::from)
    }

    fn delete(&'a self, name: &str) -> Result<()> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| self.delete(id).map_err(From::from))
            .map_err(From::from)
    }

}

pub trait BookmarkCollection : Sized + InternalLinker + ExternalLinker {
    fn links<'a>(&self, store: &'a Store)                        -> Result<UrlIter<'a>>;
    fn link_entries(&self)                                   -> Result<Vec<StoreLink>>;
    fn add_link(&mut self, store: &Store, l: Link)           -> Result<()>;
    fn get_links_matching<'a>(&self, store: &'a Store, r: Regex) -> Result<LinksMatchingRegexIter<'a>>;
    fn remove_link(&mut self, store: &Store, l: Link)        -> Result<()>;
}

impl BookmarkCollection for Entry {

    fn links<'a>(&self, store: &'a Store) -> Result<UrlIter<'a>> {
        self.get_external_links(store).map_err(From::from)
    }

    fn link_entries(&self) -> Result<Vec<StoreLink>> {
        use libimagentrylink::external::is_external_link_storeid;

        self.get_internal_links()
            .map(|v| v.filter(|id| is_external_link_storeid(id)).collect())
            .map_err(From::from)
    }

    fn add_link(&mut self, store: &Store, l: Link) -> Result<()> {
        use link::IntoUrl;

        l.into_url()
            .and_then(|url| self.add_external_link(store, url).map_err(From::from))
            .map_err(From::from)
    }

    fn get_links_matching<'a>(&self, store: &'a Store, r: Regex) -> Result<LinksMatchingRegexIter<'a>> {
        use self::iter::IntoLinksMatchingRegexIter;

        self.get_external_links(store)
            .map(|iter| iter.matching_regex(r))
            .map_err(From::from)
    }

    fn remove_link(&mut self, store: &Store, l: Link) -> Result<()> {
        use link::IntoUrl;

        l.into_url()
            .and_then(|url| self.remove_external_link(store, url).map_err(From::from))
            .map_err(From::from)
    }

}

pub mod iter {
    use link::Link;
    use error::Result;
    use error::BookmarkError as BE;

    pub struct LinkIter<I>(I)
        where I: Iterator<Item = Link>;

    impl<I: Iterator<Item = Link>> LinkIter<I> {
        pub fn new(i: I) -> LinkIter<I> {
            LinkIter(i)
        }
    }

    impl<I: Iterator<Item = Link>> Iterator for LinkIter<I> {
        type Item = Link;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    impl<I> From<I> for LinkIter<I> where I: Iterator<Item = Link> {
        fn from(i: I) -> LinkIter<I> {
            LinkIter(i)
        }
    }

    use libimagentrylink::external::iter::UrlIter;
    use regex::Regex;

    pub struct LinksMatchingRegexIter<'a>(UrlIter<'a>, Regex);

    impl<'a> LinksMatchingRegexIter<'a> {
        pub fn new(i: UrlIter<'a>, r: Regex) -> LinksMatchingRegexIter<'a> {
            LinksMatchingRegexIter(i, r)
        }
    }

    impl<'a> Iterator for LinksMatchingRegexIter<'a> {
        type Item = Result<Link>;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let n = match self.0.next() {
                    Some(Ok(n))  => n,
                    Some(Err(e)) => return Some(Err(BE::from(e))),
                    None         => return None,
                };

                let s = n.into_string();
                if self.1.is_match(&s[..]) {
                    return Some(Ok(Link::from(s)))
                } else {
                    continue;
                }
            }
        }
    }

    pub trait IntoLinksMatchingRegexIter<'a> {
        fn matching_regex(self, Regex) -> LinksMatchingRegexIter<'a>;
    }

    impl<'a> IntoLinksMatchingRegexIter<'a> for UrlIter<'a> {
        fn matching_regex(self, r: Regex) -> LinksMatchingRegexIter<'a> {
            LinksMatchingRegexIter(self, r)
        }
    }

}

