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

//! BookmarkCollection module
//!
//! A BookmarkCollection is nothing more than a simple store entry. One can simply call functions
//! from the libimagentrylink::external::ExternalLinker trait on this to generate external links.
//!
//! The BookmarkCollection type offers helper functions to get all links or such things.
use std::ops::Deref;
use std::ops::DerefMut;

use regex::Regex;

use error::BookmarkErrorKind as BEK;
use error::MapErrInto;
use result::Result;
use module_path::ModuleEntryPath;

use libimagstore::store::Store;
use libimagstore::storeid::IntoStoreId;
use libimagstore::store::FileLockEntry;
use libimagentrylink::external::ExternalLinker;
use libimagentrylink::external::iter::UrlIter;
use libimagentrylink::internal::InternalLinker;
use libimagentrylink::internal::Link as StoreLink;
use libimagerror::into::IntoError;

use link::Link;

use self::iter::LinksMatchingRegexIter;

pub struct BookmarkCollection<'a> {
    fle: FileLockEntry<'a>,
    store: &'a Store,
}

/// {Internal, External}Linker is implemented as Deref is implemented
impl<'a> Deref for BookmarkCollection<'a> {
    type Target = FileLockEntry<'a>;

    fn deref(&self) -> &FileLockEntry<'a> {
        &self.fle
    }

}

impl<'a> DerefMut for BookmarkCollection<'a> {

    fn deref_mut(&mut self) -> &mut FileLockEntry<'a> {
        &mut self.fle
    }

}

impl<'a> BookmarkCollection<'a> {

    pub fn new(store: &'a Store, name: &str) -> Result<BookmarkCollection<'a>> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| store.create(id))
            .map(|fle| {
                BookmarkCollection {
                    fle: fle,
                    store: store,
                }
            })
            .map_err_into(BEK::StoreReadError)
    }

    pub fn get(store: &'a Store, name: &str) -> Result<BookmarkCollection<'a>> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| store.get(id))
            .map_err_into(BEK::StoreReadError)
            .and_then(|fle| {
                match fle {
                    None => Err(BEK::CollectionNotFound.into_error()),
                    Some(e) => Ok(BookmarkCollection {
                        fle: e,
                        store: store,
                    }),
                }
            })
    }

    pub fn delete(store: &Store, name: &str) -> Result<()> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| store.delete(id))
            .map_err_into(BEK::StoreReadError)
    }

    pub fn links(&self) -> Result<UrlIter> {
        self.fle.get_external_links(&self.store).map_err_into(BEK::LinkError)
    }

    pub fn link_entries(&self) -> Result<Vec<StoreLink>> {
        use libimagentrylink::external::is_external_link_storeid;

        self.fle
            .get_internal_links()
            .map(|v| v.filter(|id| is_external_link_storeid(id)).collect())
            .map_err_into(BEK::StoreReadError)
    }

    pub fn add_link(&mut self, l: Link) -> Result<()> {
        use link::IntoUrl;

        l.into_url()
            .and_then(|url| self.add_external_link(self.store, url).map_err_into(BEK::LinkingError))
            .map_err_into(BEK::LinkError)
    }

    pub fn get_links_matching(&self, r: Regex) -> Result<LinksMatchingRegexIter<'a>> {
        use self::iter::IntoLinksMatchingRegexIter;

        self.get_external_links(self.store)
            .map_err_into(BEK::LinkError)
            .map(|iter| iter.matching_regex(r))
    }

    pub fn remove_link(&mut self, l: Link) -> Result<()> {
        use link::IntoUrl;

        l.into_url()
            .and_then(|url| {
                self.remove_external_link(self.store, url).map_err_into(BEK::LinkingError)
            })
            .map_err_into(BEK::LinkError)
    }

}

pub mod iter {
    use link::Link;
    use result::Result;
    use error::{MapErrInto, BookmarkErrorKind as BEK};

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
                    Some(Err(e)) => return Some(Err(e).map_err_into(BEK::LinkError)),
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

