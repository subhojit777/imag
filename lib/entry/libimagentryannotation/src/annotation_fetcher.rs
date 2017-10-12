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

use libimagstore::store::Entry;
use libimagstore::store::Store;
use libimagentrylink::internal::InternalLinker;
use libimagstore::storeid::StoreIdIterator;

use error::Result;
use error::ResultExt;

use self::iter::*;

pub trait AnnotationFetcher<'a> {

    fn all_annotations(&'a self) -> Result<AnnotationIter<'a>>;

    fn annotations_for_entry(&'a self, entry: &Entry) -> Result<AnnotationIter<'a>>;

}

impl<'a> AnnotationFetcher<'a> for Store {

    fn all_annotations(&'a self) -> Result<AnnotationIter<'a>> {
        self.retrieve_for_module("annotations")
            .map(|iter| AnnotationIter::new(iter, self))
            .map_err(Into::into)
    }

    /// Get all annotations (in an iterator) for an entry
    ///
    /// Internally, this fetches the links of the entry, fetches all the entries behind the links
    /// and filters them for annotations.
    ///
    /// This might result in some heavy IO operations if a lot of stuff is linked to a single
    /// entry, but should normally be not that heavy.
    fn annotations_for_entry(&'a self, entry: &Entry) -> Result<AnnotationIter<'a>> {
        entry.get_internal_links()
            .map_err(Into::into)
            .map(|iter| StoreIdIterator::new(Box::new(iter.map(|e| e.get_store_id().clone()))))
            .map(|i| AnnotationIter::new(i, self))
    }

}

pub mod iter {
    use toml::Value;
    use toml_query::read::TomlValueReadExt;

    use libimagstore::store::Store;
    use libimagstore::store::FileLockEntry;
    use libimagstore::storeid::StoreIdIterator;

    use error::Result;
    use error::AnnotationErrorKind as AEK;
    use error::AnnotationError as AE;
    use error::ResultExt;

    #[derive(Debug)]
    pub struct AnnotationIter<'a>(StoreIdIterator, &'a Store);

    impl<'a> AnnotationIter<'a> {

        pub fn new(iter: StoreIdIterator, store: &'a Store) -> AnnotationIter<'a> {
            AnnotationIter(iter, store)
        }

    }

    impl<'a> Iterator for AnnotationIter<'a> {
        type Item = Result<FileLockEntry<'a>>;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.0.next().map(|id| self.1.get(id)) {
                    Some(Ok(Some(entry))) => {
                        match entry.get_header().read("annotation.is_annotation") {
                            Ok(None) => continue, // not an annotation
                            Ok(Some(&Value::Boolean(true))) => return Some(Ok(entry)),
                            Ok(Some(_)) => return Some(Err(AE::from_kind(AEK::HeaderTypeError))),
                            Err(e) => return Some(Err(e).chain_err(|| AEK::HeaderReadError)),
                        }
                    },
                    Some(Ok(None)) => continue,
                    Some(Err(e))   => return Some(Err(e).chain_err(|| AEK::StoreReadError)),
                    None           => return None, // iterator consumed
                }
            }
        }

    }

}

