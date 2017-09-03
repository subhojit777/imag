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

use std::collections::BTreeMap;
use std::ops::Deref;

use toml::Value;

use libimagrt::runtime::Runtime;
use libimagentryedit::edit::Edit;
use libimagentryedit::result::Result as EditResult;
use libimagstore::storeid::IntoStoreId;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;

use toml_query::read::TomlValueReadExt;
use toml_query::set::TomlValueSetExt;

use module_path::ModuleEntryPath;
use result::Result;
use error::NoteErrorKind as NEK;
use error::NoteError as NE;
use error::ResultExt;

#[derive(Debug)]
pub struct Note<'a> {
    entry: FileLockEntry<'a>,
}

impl<'a> Note<'a> {

    pub fn new(store: &Store, name: String, text: String) -> Result<Note> {
        use std::ops::DerefMut;

        debug!("Creating new Note: '{}'", name);
        let fle = {
            let mut lockentry = try!(ModuleEntryPath::new(name.clone())
                .into_storeid()
                .and_then(|id| store.create(id))
                .chain_err(|| NEK::StoreWriteError));

            {
                let entry  = lockentry.deref_mut();

                {
                    let header = entry.get_header_mut();
                    let _ = header
                        .set("note", Value::Table(BTreeMap::new()))
                        .chain_err(|| NEK::StoreWriteError);

                    let _ = header
                        .set("note.name", Value::String(name))
                        .chain_err(|| NEK::StoreWriteError);
                }

                *entry.get_content_mut() = text;
            }

            lockentry
        };

        Ok(Note { entry: fle })
    }

    pub fn set_name(&mut self, n: String) -> Result<()> {
        self.entry
            .get_header_mut()
            .set("note.name", Value::String(n))
            .chain_err(|| NEK::StoreWriteError)
            .map(|_| ())
    }

    pub fn get_name(&self) -> Result<String> {
        let header = self.entry.get_header();
        match header.read("note.name") {
            Ok(Some(&Value::String(ref s))) => Ok(s.clone()),
            Ok(_) => {
                Err(NE::from_kind(NEK::HeaderTypeError)).chain_err(|| NEK::StoreReadError)
            },
            Err(e) => Err(e).chain_err(|| NEK::StoreReadError)
        }
    }

    pub fn set_text(&mut self, n: String) {
        *self.entry.get_content_mut() = n
    }

    pub fn get_text(&self) -> &String {
        self.entry.get_content()
    }

    pub fn delete(store: &Store, name: String) -> Result<()> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| store.delete(id))
            .chain_err(|| NEK::StoreWriteError)
    }

    pub fn retrieve(store: &Store, name: String) -> Result<Note> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| store.retrieve(id))
            .chain_err(|| NEK::StoreWriteError)
            .map(|entry| Note { entry: entry })
    }

    pub fn get(store: &Store, name: String) -> Result<Option<Note>> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| store.get(id))
            .chain_err(|| NEK::StoreWriteError)
            .map(|o| o.map(|entry| Note { entry: entry }))
    }

    pub fn all_notes(store: &Store) -> Result<NoteIterator> {
        store.retrieve_for_module("notes")
            .map(|iter| NoteIterator::new(store, iter))
            .chain_err(|| NEK::StoreReadError)
    }

}

impl<'a> Edit for Note<'a> {

    fn edit_content(&mut self, rt: &Runtime) -> EditResult<()> {
        self.entry.edit_content(rt)
    }

}

trait FromStoreId {
    fn from_storeid(&Store, StoreId) -> Result<Note>;
}

impl<'a> FromStoreId for Note<'a> {

    fn from_storeid(store: &Store, id: StoreId) -> Result<Note> {
        debug!("Loading note from storeid: '{:?}'", id);
        match store.retrieve(id) {
            Err(e)    => Err(e).chain_err(|| NEK::StoreReadError),
            Ok(entry) => Ok(Note { entry: entry }),
        }
    }

}

impl<'a> Deref for Note<'a> {

    type Target = FileLockEntry<'a>;

    fn deref(&self) -> &FileLockEntry<'a> {
        &self.entry
    }

}

#[derive(Debug)]
pub struct NoteIterator<'a> {
    store: &'a Store,
    iditer: StoreIdIterator,
}

impl<'a> NoteIterator<'a> {

    pub fn new(store: &'a Store, iditer: StoreIdIterator) -> NoteIterator<'a> {
        NoteIterator {
            store: store,
            iditer: iditer,
        }
    }

}

impl<'a> Iterator for NoteIterator<'a> {
    type Item = Result<Note<'a>>;

    fn next(&mut self) -> Option<Result<Note<'a>>> {
        self.iditer
            .next()
            .map(|id| Note::from_storeid(self.store, id))
    }

}

