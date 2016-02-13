use std::convert::Into;
use std::collections::BTreeMap;
use std::ops::{DerefMut, Deref};

use toml::Value;

use libimagstore::storeid::IntoStoreId;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagtag::tag::Tag;
use libimagtag::tagable::Tagable;
use libimagtag::result::Result as TagResult;
use libimagtag::error::{TagError, TagErrorKind};

use module_path::ModuleEntryPath;
use result::Result;
use error::NoteError as NE;
use error::NoteErrorKind as NEK;

pub struct Note<'a> {
    entry: FileLockEntry<'a>,
}

impl<'a> Note<'a> {

    pub fn new(store: &Store, name: String, text: String) -> Result<Note> {
        use std::ops::DerefMut;

        debug!("Creating new Note: '{}'", name);
        let fle = {
            let mut lockentry = store.create(ModuleEntryPath::new(name.clone()).into_storeid());
            if lockentry.is_err() {
                return Err(NE::new(NEK::StoreWriteError, Some(Box::new(lockentry.err().unwrap()))));
            }
            let mut lockentry = lockentry.unwrap();

            {
                let mut entry  = lockentry.deref_mut();
                let mut header = entry.get_header_mut();
                let setres = header.set("note", Value::Table(BTreeMap::new()));
                if setres.is_err() {
                    return Err(NE::new(NEK::StoreWriteError, Some(Box::new(setres.err().unwrap()))));
                }

                let setres = header.set("note.name", Value::String(name));
                if setres.is_err() {
                    return Err(NE::new(NEK::StoreWriteError, Some(Box::new(setres.err().unwrap()))));
                }
            }

            lockentry
        };

        Ok(Note { entry: fle })
    }

    pub fn set_name(&mut self, n: String) -> Result<()> {
        let mut header = self.entry.deref_mut().get_header_mut();
        header.set("note.name", Value::String(n))
            .map_err(|e| NE::new(NEK::StoreWriteError, Some(Box::new(e))))
            .map(|_| ())
    }

    pub fn get_name(&self) -> Result<String> {
        let mut header = self.entry.deref().get_header();
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
        *self.entry.deref_mut().get_content_mut() = n
    }

    pub fn get_text(&self) -> &String {
        self.entry.deref().get_content()
    }

}

impl<'a> Tagable for Note<'a> {

    fn get_tags(&self) -> TagResult<Vec<Tag>> {
        self.entry.deref().get_tags()
    }

    fn set_tags(&mut self, ts: Vec<Tag>) -> TagResult<()> {
        self.entry.deref_mut().set_tags(ts)
    }

    fn add_tag(&mut self, t: Tag) -> TagResult<()> {
        self.entry.deref_mut().add_tag(t)
    }

    fn remove_tag(&mut self, t: Tag) -> TagResult<()> {
        self.entry.deref_mut().remove_tag(t)
    }

    fn has_tag(&self, t: &Tag) -> TagResult<bool> {
        self.entry.deref().has_tag(t)
    }

    fn has_tags(&self, ts: &Vec<Tag>) -> TagResult<bool> {
        self.entry.deref().has_tags(ts)
    }

}

