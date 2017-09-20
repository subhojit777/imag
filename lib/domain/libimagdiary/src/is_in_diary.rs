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

use libimagstore::store::Entry;
use libimagstore::storeid::StoreId;

pub trait IsInDiary {

    fn is_in_diary(&self, name: &str) -> bool;

    fn is_a_diary_entry(&self) -> bool;

}

impl IsInDiary for Entry {

    fn is_in_diary(&self, name: &str) -> bool {
        self.get_location().clone().is_in_diary(name)
    }

    fn is_a_diary_entry(&self) -> bool {
        self.get_location().clone().is_a_diary_entry()
    }

}

impl IsInDiary for StoreId {

    fn is_in_diary(&self, name: &str) -> bool {
        self.is_in_collection(&["diary", name])
    }

    fn is_a_diary_entry(&self) -> bool {
        self.is_in_collection(&["diary"])
    }

}

