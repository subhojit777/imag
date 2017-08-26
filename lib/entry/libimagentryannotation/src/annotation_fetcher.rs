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
use libimagnotes::note::Note;
use libimagnotes::note::NoteIterator;
use libimagstore::storeid::StoreIdIterator;

use result::Result;
use error::AnnotationErrorKind as AEK;
use error::MapErrInto;

use self::iter::*;

pub trait AnnotationFetcher<'a> {

    fn all_annotations(&'a self) -> Result<AnnotationIter<'a>>;

    fn annotations_for_entry(&'a self, entry: &Entry) -> Result<AnnotationIter<'a>>;

}

impl<'a> AnnotationFetcher<'a> for Store {

    /// Wrapper around `Note::all_notes()` of `libimagnotes` which filters out normal notes and
    /// leaves only annotations in the iterator.
    fn all_annotations(&'a self) -> Result<AnnotationIter<'a>> {
        Note::all_notes(self)
            .map(|iter| AnnotationIter::new(iter))
            .map_err_into(AEK::StoreReadError)
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
            .map_err_into(AEK::StoreReadError)
            .map(|iter| StoreIdIterator::new(Box::new(iter.map(|e| e.get_store_id().clone()))))
            .map(|iter| NoteIterator::new(self, iter))
            .map(|iter| AnnotationIter::new(iter))
    }

}

pub mod iter {
    use toml::Value;

    use toml_query::read::TomlValueReadExt;

    use libimagerror::into::IntoError;
    use libimagnotes::note::Note;
    use libimagnotes::note::NoteIterator;

    use result::Result;
    use error::AnnotationErrorKind as AEK;
    use error::MapErrInto;

    #[derive(Debug)]
    pub struct AnnotationIter<'a>(NoteIterator<'a>);

    impl<'a> AnnotationIter<'a> {

        pub fn new(noteiter: NoteIterator<'a>) -> AnnotationIter<'a> {
            AnnotationIter(noteiter)
        }

    }

    impl<'a> Iterator for AnnotationIter<'a> {
        type Item = Result<Note<'a>>;

        fn next(&mut self) -> Option<Result<Note<'a>>> {
            loop {
                match self.0.next() {
                    Some(Ok(note)) => {
                        match note.get_header().read("annotation.is_annotation") {
                            Ok(None) => continue, // not an annotation
                            Ok(Some(&Value::Boolean(true))) => return Some(Ok(note)),
                            Ok(Some(_)) => return Some(Err(AEK::HeaderTypeError.into_error())),
                            Err(e) => return Some(Err(e).map_err_into(AEK::HeaderReadError)),
                        }
                    },
                    Some(Err(e)) => return Some(Err(e).map_err_into(AEK::StoreReadError)),
                    None => return None, // iterator consumed
                }
            }
        }

    }

}

