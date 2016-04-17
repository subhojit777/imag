use std::ops::Deref;
use std::ops::DerefMut;

use libimagstore::store::FileLockEntry;
use libimagrt::edit::Edit;
use libimagrt::edit::EditResult;
use libimagrt::runtime::Runtime;

#[derive(Debug)]
pub struct Entry<'a>(FileLockEntry<'a>);

impl<'a> Deref for Entry<'a> {
    type Target = FileLockEntry<'a>;

    fn deref(&self) -> &FileLockEntry<'a> {
        &self.0
    }

}

impl<'a> DerefMut for Entry<'a> {

    fn deref_mut(&mut self) -> &mut FileLockEntry<'a> {
        &mut self.0
    }

}

impl<'a> Entry<'a> {

    pub fn new(fle: FileLockEntry<'a>) -> Entry<'a> {
        Entry(fle)
    }

    /// Get the diary id for this entry.
    ///
    /// TODO: calls Option::unwrap() as it assumes that an existing Entry has an ID that is parsable
    pub fn diary_id(&self) -> DiaryId {
        DiaryId::from_storeid(&self.0.get_location().clone()).unwrap()
    }

}

impl<'a> Into<FileLockEntry<'a>> for Entry<'a> {

    fn into(self) -> FileLockEntry<'a> {
        self.0
    }

}

impl<'a> From<FileLockEntry<'a>> for Entry<'a> {

    fn from(fle: FileLockEntry<'a>) -> Entry<'a> {
        Entry::new(fle)
    }

}

impl<'a> Edit for Entry<'a> {

    fn edit_content(&mut self, rt: &Runtime) -> EditResult<()> {
        self.0.edit_content(rt)
    }

}


