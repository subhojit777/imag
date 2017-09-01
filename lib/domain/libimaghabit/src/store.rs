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

use error::Result;
use habit::builder::HabitBuilder;
use iter::HabitTemplateStoreIdIterator;

use libimagstore::store::Store;

/// Extension trait for libimagstore::store::Store which is basically our Habit-Store
pub trait HabitStore {

    /// Create a new habit
    fn create_habit(&self) -> HabitBuilder {
        HabitBuilder::default()
    }

    /// Get an iterator over all habits
    fn all_habit_templates(&self) -> Result<HabitTemplateStoreIdIterator>;

}

impl HabitStore for Store {
    /// Get an iterator over all habits
    fn all_habit_templates(&self) -> Result<HabitTemplateStoreIdIterator> {
        self.entries().map(HabitTemplateStoreIdIterator::from).map_err(From::from)
    }
}
