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
use libimagstore::toml_ext::TomlValueExt;
use libimagentrylink::internal::InternalLinker;
use libimagerror::into::IntoError;

use result::Result;
use error::AnnotationErrorKind as AEK;
use error::MapErrInto;

pub trait Annotateable {

    /// Add an annotation to `Self`, that is a `FileLockEntry` which is linked to `Self` (link as in
    /// libimagentrylink).
    ///
    /// A new annotation also has the field `annotation.is_annotation` set to `true`.
    fn annotate<'a>(&mut self, store: &'a Store, ann_name: &str) -> Result<FileLockEntry<'a>>;

}

impl Annotateable for Entry {

    fn annotate<'a>(&mut self, store: &'a Store, ann_name: &str) -> Result<FileLockEntry<'a>> {
        store.retrieve(PathBuf::from(ann_name))
            .map_err_into(AEK::StoreWriteError)
            .and_then(|mut anno| {
                anno.get_header_mut()
                    .insert("annotation.is_annotation", Value::Boolean(true))
                    .map_err_into(AEK::StoreWriteError)
                    .and_then(|succeeded| {
                        if succeeded {
                            Ok(anno)
                        } else {
                            Err(AEK::HeaderWriteError.into_error())
                        }
                    })
            })
            .and_then(|mut anno| {
                anno.add_internal_link(self)
                    .map_err_into(AEK::LinkingError)
                    .map(|_| anno)
            })
    }

}

