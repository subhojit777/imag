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

use libimagstore::store::Result as StoreResult;
use libimagstore::storeid::IntoStoreId;
use libimagstore::storeid::StoreId;

/// A tag for time-tracking. This is not a normal `libimagentrytag` tag, because we want the user
/// give the possibility to use the tagging functionality without interfering with this functionality.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeTrackingTag(String);

impl TimeTrackingTag {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Into<String> for TimeTrackingTag {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for TimeTrackingTag {
    fn from(s: String) -> TimeTrackingTag {
        TimeTrackingTag(s)
    }
}

impl<'a> From<&'a String> for TimeTrackingTag {
    fn from(s: &'a String) -> TimeTrackingTag {
        TimeTrackingTag(s.clone())
    }
}

impl IntoStoreId for TimeTrackingTag {
    fn into_storeid(self) -> StoreResult<StoreId> {
        StoreId::new_baseless(PathBuf::from(self.0))
    }
}

