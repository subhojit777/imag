use std::collections::BTreeMap;
use std::ops::{DerefMut, Deref};

use toml::Value;

use libimagrt::runtime::Runtime;
use libimagrt::edit::{Edit, EditResult};
use libimagstore::storeid::IntoStoreId;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagentrytag::tag::Tag;
use libimagentrytag::tagable::Tagable;
use libimagentrytag::result::Result as TagResult;

use module_path::ModuleEntryPath;
use result::Result;
use error::NoteError as NE;
use error::NoteErrorKind as NEK;

#[derive(Debug)]
pub struct Note<'a> {
    entry: FileLockEntry<'a>,
}

impl<'a> Note<'a> {

    pub fn new(store: &Store, name: String, text: String) -> Result<Note> {
        use std::ops::DerefMut;

        debug!("Creating new Note: '{}'", name);
        let fle = {
            let lockentry = store.create(ModuleEntryPath::new(name.clone()).into_storeid());
            if lockentry.is_err() {
                return Err(NE::new(NEK::StoreWriteError, Some(Box::new(lockentry.unwrap_err()))));
            }
            let mut lockentry = lockentry.unwrap();

            {
                let mut entry  = lockentry.deref_mut();

                {
                    let mut header = entry.get_header_mut();
                    let setres = header.set("note", Value::Table(BTreeMap::new()));
                    if setres.is_err() {
                        let kind = NEK::StoreWriteError;
                        return Err(NE::new(kind, Some(Box::new(setres.unwrap_err()))));
                    }

                    let setres = header.set("note.name", Value::String(name));
                    if setres.is_err() {
                        let kind = NEK::StoreWriteError;
                        return Err(NE::new(kind, Some(Box::new(setres.unwrap_err()))));
                    }
                }

                *entry.get_content_mut() = text;
            }

            lockentry
        };

        Ok(Note { entry: fle })
    }

    pub fn set_name(&mut self, n: String) -> Result<()> {
        let mut header = self.entry.get_header_mut();
        header.set("note.name", Value::String(n))
            .map_err(|e| NE::new(NEK::StoreWriteError, Some(Box::new(e))))
            .map(|_| ())
    }

    pub fn get_name(&self) -> Result<String> {
        let header = self.entry.get_header();
        match header.read("note.name") {
            Ok(Some(Value::String(s))) => Ok(String::from(s)),
            Ok(_)                => {
                let e = NE::new(NEK::HeaderTypeError, None);
                Err(NE::new(NEK::StoreReadError, Some(Box::new(e))))
            },
            Err(e) => Err(NE::new(NEK::StoreReadError, Some(Box::new(e))))
        }
    }

    pub fn set_text(&mut self, n: String) {
        *self.entry.get_content_mut() = n
    }

    pub fn get_text(&self) -> &String {
        self.entry.get_content()
    }

    pub fn delete(store: &Store, name: String) -> Result<()> {
        store.delete(ModuleEntryPath::new(name).into_storeid())
            .map_err(|e| NE::new(NEK::StoreWriteError, Some(Box::new(e))))
    }

    pub fn retrieve(store: &Store, name: String) -> Result<Note> {
        store.retrieve(ModuleEntryPath::new(name).into_storeid())
            .map_err(|e| NE::new(NEK::StoreWriteError, Some(Box::new(e))))
            .map(|entry| Note { entry: entry })
    }

    pub fn all_notes(store: &Store) -> Result<NoteIterator> {
        store.retrieve_for_module("notes")
            .map(|iter| NoteIterator::new(store, iter))
            .map_err(|e| NE::new(NEK::StoreReadError, Some(Box::new(e))))
    }

}

impl<'a> Edit for Note<'a> {

    fn edit_content(&mut self, rt: &Runtime) -> EditResult<()> {
        self.entry.edit_content(rt)
    }

}

impl<'a> Tagable for Note<'a> {

    fn get_tags(&self) -> TagResult<Vec<Tag>> {
        self.entry.get_tags()
    }

    fn set_tags(&mut self, ts: Vec<Tag>) -> TagResult<()> {
        self.entry.set_tags(ts)
    }

    fn add_tag(&mut self, t: Tag) -> TagResult<()> {
        self.entry.add_tag(t)
    }

    fn remove_tag(&mut self, t: Tag) -> TagResult<()> {
        self.entry.remove_tag(t)
    }

    fn has_tag(&self, t: &Tag) -> TagResult<bool> {
        self.entry.has_tag(t)
    }

    fn has_tags(&self, ts: &Vec<Tag>) -> TagResult<bool> {
        self.entry.has_tags(ts)
    }

}

trait FromStoreId {
    fn from_storeid<'a>(&'a Store, StoreId) -> Result<Note<'a>>;
}

impl<'a> FromStoreId for Note<'a> {

    fn from_storeid<'b>(store: &'b Store, id: StoreId) -> Result<Note<'b>> {
        debug!("Loading note from storeid: '{:?}'", id);
        match store.retrieve(id) {
            Err(e)    => Err(NE::new(NEK::StoreReadError, Some(Box::new(e)))),
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

