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

use libimagstore::store::Entry;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;

use diaryid::DiaryId;
use diaryid::FromStoreId;
use error::Result;

provide_kindflag_path!(pub IsDiaryEntry, "diary.is_diary_entry");

pub trait DiaryEntry {
    fn is_diary_entry(&self) -> Result<bool>;
    fn diary_id(&self) -> Result<DiaryId>;
}

impl DiaryEntry for Entry {

    /// Check whether the entry is a diary entry by checking its headers
    fn is_diary_entry(&self) -> Result<bool> {
        self.is::<IsDiaryEntry>().map_err(From::from)
    }

    /// Get the diary id for this entry.
    ///
    /// TODO: calls Option::unwrap() as it assumes that an existing Entry has an ID that is parsable
    fn diary_id(&self) -> Result<DiaryId> {
        DiaryId::from_storeid(&self.get_location().clone())
    }

}

