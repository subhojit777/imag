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
use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::set::TomlValueSetExt;

use error::HabitErrorKind as HEK;
use error::*;
use util::*;

use libimagstore::store::Entry;

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
    fn set_date(&mut self, n: &NaiveDate) -> Result<()>;
    fn get_comment(&self) -> Result<String>;
    fn set_comment(&mut self, c: String) -> Result<()>;
    fn get_template_name(&self) -> Result<String>;
}

impl HabitInstance for Entry {
    fn is_habit_instance(&self) -> Result<bool> {
        [
            "habit.instance.name",
            "habit.instance.date",
            "habit.instance.comment",
        ].iter().fold(Ok(true), |acc, path| acc.and_then(|_| {
            self.get_header()
                .read(path)
                .map(|o| is_match!(o, Some(&Value::String(_))))
                .map_err(From::from)
        }))
    }

    fn get_date(&self) -> Result<NaiveDate> {
        match self.get_header().read("habit.instance.date")? {
            Some(&Value::String(ref s)) => date_from_string(s),
            Some(_) => Err(HEK::HeaderTypeError("habit.instance.date", "String").into()),
            None    => Err(HEK::HeaderFieldMissing("habit.instance.date").into()),
        }
    }

    fn set_date(&mut self, n: &NaiveDate) -> Result<()> {
        // Using `set` here because when creating the entry, these headers should be made present.
        self.get_header_mut()
            .set("habit.instance.date", Value::String(date_to_string(n)))
            .map_err(From::from)
            .map(|_| ())
    }

    fn get_comment(&self) -> Result<String> {
        match self.get_header().read("habit.instance.comment")? {
            Some(&Value::String(ref s)) => Ok(s.clone()),
            Some(_) => Err(HEK::HeaderTypeError("habit.instance.comment", "String").into()),
            None    => Err(HEK::HeaderFieldMissing("habit.instance.comment").into()),
        }
    }

    fn set_comment(&mut self, c: String) -> Result<()> {
        // Using `set` here because when creating the entry, these headers should be made present.
        self.get_header_mut()
            .set("habit.instance.comment", Value::String(c))
            .map_err(From::from)
            .map(|_| ())
    }

    fn get_template_name(&self) -> Result<String> {
        match self.get_header().read("habit.instance.name")? {
            Some(&Value::String(ref s)) => Ok(s.clone()),
            Some(_) => Err(HEK::HeaderTypeError("habit.instance.name", "String").into()),
            None    => Err(HEK::HeaderFieldMissing("habit.instance.name").into()),
        }
    }

}
