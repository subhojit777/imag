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

pub const NAIVE_DATE_STRING_FORMAT : &'static str = "%Y-%m-%d";

pub fn date_to_string(ndt: &NaiveDate) -> String {
    ndt.format(NAIVE_DATE_STRING_FORMAT).to_string()
}

pub fn date_from_string(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, NAIVE_DATE_STRING_FORMAT).map_err(From::from)
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

