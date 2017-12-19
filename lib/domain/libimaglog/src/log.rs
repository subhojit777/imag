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

use libimagdiary::entry::DiaryEntry;
use libimagstore::store::Entry;

use error::LogError as LE;
use error::LogErrorKind as LEK;
use error::Result;

use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;

pub trait Log : DiaryEntry {
    fn is_log(&self) -> Result<bool>;
    fn make_log_entry(&mut self) -> Result<()>;
}

impl Log for Entry {
    fn is_log(&self) -> Result<bool> {
        let location = "log.is_log";
        match self.get_header().read(location)? {
            Some(&Value::Boolean(b)) => Ok(b),
            Some(_) => Err(LE::from_kind(LEK::HeaderTypeError("boolean", location))),
            None    => Ok(false)
        }
    }

    fn make_log_entry(&mut self) -> Result<()> {
        self.get_header_mut()
            .insert("log.is_log", Value::Boolean(true))
            .map_err(LE::from)
            .map(|_| ())
    }

}

