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

use chrono::NaiveDate;

use error::Result;
use habit::HabitTemplate;

/// An instance of a habit is created for each time a habit is done.
///
/// # Note
///
/// A habit is a daily thing, so we only provide "date" as granularity for its time data.
///
pub trait HabitInstance {
    /// Check whether the instance is a habit instance by checking its headers for the habit
    /// data
    fn is_habit_instance(&self) -> Result<bool>;

    fn get_date(&self) -> Result<NaiveDate>;
    fn set_date(&self, n: NaiveDate) -> Result<()>;
    fn get_comment(&self) -> Result<String>;
    fn set_comment(&self, c: String) -> Result<()>;
    fn get_template_name(&self) -> Result<String>;
}

