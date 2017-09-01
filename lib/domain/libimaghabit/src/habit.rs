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

use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;
use chrono::NaiveDateTime;
use chrono::Local;
use chrono::NaiveDate;

use error::HabitError as HE;
use error::HabitErrorKind as HEK;
use error::*;
use iter::HabitInstanceStoreIdIterator;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Entry;
use libimagstore::iter::get::StoreIdGetIteratorExtension;
use libimagstore::storeid::IntoStoreId;

pub const NAIVE_DATE_STRING_FORMAT : &'static str = "%Y-%m-%d";

/// A HabitTemplate is a "template" of a habit. A user may define a habit "Eat vegetable".
/// If the user ate a vegetable, she should create a HabitInstance from the Habit with the
/// appropriate date (and optionally a comment) set.
pub trait HabitTemplate : Sized {

    /// Create an instance from this habit template
    ///
    /// By default creates an instance with the name of the template, the current time and the
    /// current date and copies the comment from the template to the instance.
    ///
    /// It uses `Store::retrieve()` underneath
    fn create_instance<'a>(&self, store: &'a Store) -> Result<FileLockEntry<'a>>;

    /// Check whether the instance is a habit by checking its headers for the habit data
    fn is_habit_template(&self) -> Result<bool>;

    fn habit_name(&self) -> Result<String>;
    fn habit_date(&self) -> Result<String>;
    fn habit_comment(&self) -> Result<String>;

}

impl HabitTemplate for Entry {

    fn create_instance<'a>(&self, store: &'a Store) -> Result<FileLockEntry<'a>> {
        use module_path::ModuleEntryPath;
        let date = date_to_string(&Local::today().naive_local());
        let name = self.habit_name()?;
        let comment = self.habit_comment()?;
        let id = ModuleEntryPath::new(format!("instance/{}-{}", name, date))
            .into_storeid()
            .map_err(HE::from)?;

        store.retrieve(id)
            .map_err(From::from)
            .and_then(|mut entry| {
                {
                    let mut hdr = entry.get_header_mut();
                    try!(hdr.insert("habit.instance.name",    Value::String(name)));
                    try!(hdr.insert("habit.instance.date",    Value::String(date)));
                    try!(hdr.insert("habit.instance.comment", Value::String(comment)));
                }
                Ok(entry)
            })
    }

    /// Check whether the instance is a habit by checking its headers for the habit data
    fn is_habit_template(&self) -> Result<bool> {
        [
            "habit.template.name",
            "habit.template.date",
            "habit.template.comment",
        ].iter().fold(Ok(true), |acc, path| acc.and_then(|b| {
            self.get_header()
                .read(path)
                .map(|o| is_match!(o, Some(&Value::String(_))))
                .map_err(From::from)
        }))
    }

    fn habit_name(&self) -> Result<String> {
        get_string_header_from_habit(self, "habit.template.name")
    }

    fn habit_date(&self) -> Result<String> {
        get_string_header_from_habit(self, "habit.template.date")
    }

    fn habit_comment(&self) -> Result<String> {
        get_string_header_from_habit(self, "habit.template.comment")
    }

}

#[inline]
fn get_string_header_from_habit(e: &Entry, path: &'static str) -> Result<String> {
    match e.get_header().read(path)? {
        Some(&Value::String(ref s)) => Ok(s.clone()),
        Some(_) => Err(HEK::HeaderTypeError(path, "String").into()),
        None    => Err(HEK::HeaderFieldMissing(path).into()),
    }
}

pub mod builder {
    use toml::Value;
    use toml_query::insert::TomlValueInsertExt;
    use chrono::NaiveDate;

    use libimagstore::store::Store;
    use libimagstore::storeid::StoreId;
    use libimagstore::storeid::IntoStoreId;
    use libimagstore::store::FileLockEntry;

    use error::HabitError as HE;
    use error::HabitErrorKind as HEK;
    use error::*;

    use super::date_to_string;
    use super::date_from_string;

    pub struct HabitBuilder {
        name: Option<String>,
        comment: Option<String>,
        date: Option<NaiveDate>,
    }

    impl HabitBuilder {

        pub fn with_name(&mut self, name: String) -> &mut Self {
            self.name = Some(name);
            self
        }

        pub fn with_comment(&mut self, comment: String) -> &mut Self {
            self.comment = Some(comment);
            self
        }

        pub fn with_date(&mut self, date: NaiveDate) -> &mut Self {
            self.date = Some(date);
            self
        }

        pub fn build<'a>(self, store: &'a Store) -> Result<FileLockEntry<'a>> {
            #[inline]
            fn mkerr(s: &'static str) -> HE {
                HE::from_kind(HEK::HabitBuilderMissing(s))
            }

            let name      = try!(self.name.ok_or_else(|| mkerr("name")));
            let dateobj   = try!(self.date.ok_or_else(|| mkerr("date")));
            let date      = date_to_string(&dateobj);
            let comment   = self.comment.unwrap_or_else(|| String::new());
            let sid       = try!(build_habit_template_sid(&name));
            let mut entry = try!(store.create(sid));

            try!(entry.get_header_mut().insert("habit.template.name", Value::String(name)));
            try!(entry.get_header_mut().insert("habit.template.date", Value::String(date)));
            try!(entry.get_header_mut().insert("habit.template.comment", Value::String(comment)));

            Ok(entry)
        }

    }

    impl Default for HabitBuilder {
        fn default() -> Self {
            HabitBuilder {
                name: None,
                comment: None,
                date: None,
            }
        }
    }

    /// Buld a StoreId for a Habit from a date object and a name of a habit
    fn build_habit_template_sid(name: &String) -> Result<StoreId> {
        use module_path::ModuleEntryPath;
        ModuleEntryPath::new(format!("template/{}", name)).into_storeid().map_err(From::from)
    }

}

fn date_to_string(ndt: &NaiveDate) -> String {
    ndt.format(NAIVE_DATE_STRING_FORMAT).to_string()
}

fn date_from_string(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, NAIVE_DATE_STRING_FORMAT).map_err(From::from)
}

