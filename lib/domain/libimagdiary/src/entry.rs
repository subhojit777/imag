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

use std::ops::Deref;
use std::ops::DerefMut;

use libimagstore::store::FileLockEntry;
use libimagentryedit::edit::Edit;
use libimagentryedit::result::Result as EditResult;
use libimagrt::runtime::Runtime;

use diaryid::DiaryId;
use diaryid::FromStoreId;

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


