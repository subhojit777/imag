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

use std::ops::BitXor;

use chrono::NaiveDate;
use error::Result;

use habit::HabitTemplate;
use instance::HabitInstance;

use libimagstore::storeid::StoreId;
use libimagstore::store::Entry;

pub const NAIVE_DATE_STRING_FORMAT : &'static str = "%Y-%m-%d";

pub fn date_to_string(ndt: &NaiveDate) -> String {
    ndt.format(NAIVE_DATE_STRING_FORMAT).to_string()
}

pub fn date_from_string(s: String) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(&s, NAIVE_DATE_STRING_FORMAT).map_err(From::from)
}

/// Helper trait to check whether a object which can be a habit instance and a habit template is
/// actually a valid object, whereas "valid" is defined that it is _either_ an instance or a
/// template (think XOR).
pub trait IsValidHabitObj : HabitInstance + HabitTemplate {
    fn is_valid_havit_obj(&self) -> Result<bool> {
        self.is_habit_instance().and_then(|b| self.is_habit_template().map(|a| a.bitxor(b)))
    }
}

impl<H> IsValidHabitObj for H
    where H: HabitInstance + HabitTemplate
{
    // Empty
}

pub trait IsHabitCheck {
    fn is_habit(&self) -> bool;
    fn is_habit_instance(&self) -> bool;
    fn is_habit_template(&self) -> bool;
}

impl IsHabitCheck for StoreId {
    fn is_habit(&self) -> bool {
        self.is_in_collection(&["habit"])
    }

    fn is_habit_instance(&self) -> bool {
        self.is_in_collection(&["habit", "instance"])
    }

    fn is_habit_template(&self) -> bool {
        self.is_in_collection(&["habit", "template"])
    }
}

impl IsHabitCheck for Entry {
    /// Helper function to check whether an entry is a habit (either instance or template)
    fn is_habit(&self) -> bool {
        self.get_location().is_habit()
    }

    /// Check whether an entry is a habit instance
    fn is_habit_instance(&self) -> bool {
        self.get_location().is_habit_instance()
    }

    /// Check whether an entry is a habit template
    fn is_habit_template(&self) -> bool {
        self.get_location().is_habit_template()
    }
}

#[inline]
pub fn get_string_header_from_entry(e: &Entry, path: &'static str) -> Result<String> {
    use error::HabitErrorKind as HEK;
    use toml_query::read::TomlValueReadExt;

    e.get_header()
        .read(path)?
        .ok_or(HEK::HeaderFieldMissing(path).into())
        .and_then(|o| o.as_str().map(String::from).ok_or(HEK::HeaderTypeError(path, "String").into()))
}

