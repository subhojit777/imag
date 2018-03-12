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

use toml::Value;

use libimagstore::storeid::IntoStoreId;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;

use toml_query::insert::TomlValueInsertExt;

use module_path::ModuleEntryPath;
use error::Result;
use error::NoteError as NE;
use iter::*;

pub trait NoteStore<'a> {
    fn new_note(&'a self, name: String, text: String) -> Result<FileLockEntry<'a>>;
    fn delete_note(&'a self, name: String) -> Result<()>;
    fn retrieve_note(&'a self, name: String) -> Result<FileLockEntry<'a>>;
    fn get_note(&'a self, name: String) -> Result<Option<FileLockEntry<'a>>>;
    fn all_notes(&'a self) -> Result<NoteIterator>;
}


impl<'a> NoteStore<'a> for Store {

    fn new_note(&'a self, name: String, text: String) -> Result<FileLockEntry<'a>> {
        use std::ops::DerefMut;

        debug!("Creating new Note: '{}'", name);
        let fle = {
            let mut lockentry = ModuleEntryPath::new(name.clone())
                .into_storeid()
                .and_then(|id| self.create(id))?;

            {
                let entry  = lockentry.deref_mut();
                entry.get_header_mut().insert("note.name", Value::String(name))?;
                *entry.get_content_mut() = text;
            }

            lockentry
        };

        Ok(fle)
    }

    fn delete_note(&'a self, name: String) -> Result<()> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| self.delete(id))
            .map_err(NE::from)
    }

    fn retrieve_note(&'a self, name: String) -> Result<FileLockEntry<'a>> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| self.retrieve(id))
            .map_err(NE::from)
    }

    fn get_note(&'a self, name: String) -> Result<Option<FileLockEntry<'a>>> {
        ModuleEntryPath::new(name)
            .into_storeid()
            .and_then(|id| self.get(id))
            .map_err(NE::from)
    }

    fn all_notes(&'a self) -> Result<NoteIterator> {
        self.retrieve_for_module("notes")
            .map(NoteIterator::new)
            .map_err(NE::from)
    }

}

