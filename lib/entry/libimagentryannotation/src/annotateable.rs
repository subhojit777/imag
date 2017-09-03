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

use std::path::PathBuf;

use toml::Value;

use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagentrylink::internal::InternalLinker;

use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;

use result::Result;
use error::AnnotationErrorKind as AEK;
use error::AnnotationError as AE;
use error::ResultExt;

pub trait Annotateable {

    /// Add an annotation to `Self`, that is a `FileLockEntry` which is linked to `Self` (link as in
    /// libimagentrylink).
    ///
    /// A new annotation also has the field `annotation.is_annotation` set to `true`.
    fn annotate<'a>(&mut self, store: &'a Store, ann_name: &str) -> Result<FileLockEntry<'a>>;

    /// Check whether an entry is an annotation
    fn is_annotation(&self) -> Result<bool>;

}

impl Annotateable for Entry {

    fn annotate<'a>(&mut self, store: &'a Store, ann_name: &str) -> Result<FileLockEntry<'a>> {
        store.retrieve(PathBuf::from(ann_name))
            .chain_err(|| AEK::StoreWriteError)
            .and_then(|mut anno| {
                anno.get_header_mut()
                    .insert("annotation.is_annotation", Value::Boolean(true))
                    .chain_err(|| AEK::HeaderWriteError)
                    .map(|_| anno)
            })
            .and_then(|mut anno| {
                anno.add_internal_link(self)
                    .chain_err(|| AEK::LinkingError)
                    .map(|_| anno)
            })
    }

    fn is_annotation(&self) -> Result<bool> {
        self.get_header()
            .read("annotation.is_annotation")
            .chain_err(|| AEK::StoreReadError)
            .and_then(|res| match res {
                Some(&Value::Boolean(b)) => Ok(b),
                None                     => Ok(false),
                _                        => Err(AE::from_kind(AEK::HeaderTypeError)),
            })
    }

}

