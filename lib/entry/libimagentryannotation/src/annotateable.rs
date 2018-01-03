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

use toml::Value;

use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagstore::storeid::IntoStoreId;
use libimagstore::storeid::StoreIdIterator;
use libimagentrylink::internal::InternalLinker;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;

use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;

use error::Result;
use error::AnnotationErrorKind as AEK;
use error::AnnotationError as AE;
use error::ResultExt;

use iter::*;

pub trait Annotateable {
    fn annotate<'a>(&mut self, store: &'a Store, ann_name: &str) -> Result<FileLockEntry<'a>>;
    fn denotate<'a>(&mut self, store: &'a Store, ann_name: &str) -> Result<Option<FileLockEntry<'a>>>;
    fn annotations<'a>(&self, store: &'a Store) -> Result<AnnotationIter<'a>>;
    fn is_annotation(&self) -> Result<bool>;
}

provide_kindflag_path!(IsAnnotation, "annotation.is_annotation");

impl Annotateable for Entry {

    /// Annotate an entry, returns the new entry which is used to annotate
    fn annotate<'a>(&mut self, store: &'a Store, ann_name: &str) -> Result<FileLockEntry<'a>> {
        use module_path::ModuleEntryPath;
        store.retrieve(ModuleEntryPath::new(ann_name).into_storeid()?)
            .map_err(From::from)
            .and_then(|mut anno| {
                {
                    let _ = anno.set_isflag::<IsAnnotation>()?;
                    let _ = anno
                        .get_header_mut()
                        .insert("annotation.name", Value::String(String::from(ann_name)))?;
                }
                Ok(anno)
            })
            .and_then(|mut anno| {
                anno.add_internal_link(self)
                    .chain_err(|| AEK::LinkingError)
                    .map(|_| anno)
            })
    }

    /// Checks the current entry for all annotations and removes the one where the name is
    /// `ann_name`, which is then returned
    fn denotate<'a>(&mut self, store: &'a Store, ann_name: &str) -> Result<Option<FileLockEntry<'a>>> {
        for annotation in self.annotations(store)? {
            let mut anno = annotation?;
            let name = match anno.get_header().read("annotation.name")? {
                None      => continue,
                Some(val) => match *val {
                    Value::String(ref name) => name.clone(),
                    _ => return Err(AE::from_kind(AEK::HeaderTypeError)),
                },
            };

            if name == ann_name {
                let _ = self.remove_internal_link(&mut anno)?;
                return Ok(Some(anno));
            }
        }

        Ok(None)
    }

    /// Get all annotations of an entry
    fn annotations<'a>(&self, store: &'a Store) -> Result<AnnotationIter<'a>> {
        self.get_internal_links()
            .map_err(From::from)
            .map(|iter| StoreIdIterator::new(Box::new(iter.map(|e| e.get_store_id().clone()))))
            .map(|i| AnnotationIter::new(i, store))
    }

    fn is_annotation(&self) -> Result<bool> {
        self.is::<IsAnnotation>().map_err(From::from)
    }

}

