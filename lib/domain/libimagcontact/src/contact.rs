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

use std::ops::Deref;

use vobject::Component;
use toml::Value;
use toml_query::read::TomlValueReadExt;

use libimagstore::store::Entry;
use libimagentryref::reference::Ref;

use error::ContactError as CE;
use error::ContactErrorKind as CEK;
use error::Result;
use util;

/// Trait to be implemented on ::libimagstore::store::Entry
///
/// Based on the functionality from libimagentryref, for fetching the Ical data from disk
pub trait Contact : Ref {

    fn is_contact(&self) -> Result<bool>;

    // getting data

    fn get_contact_data(&self) -> Result<ContactData>;

    // More convenience functionality may follow

}

impl Contact for Entry {

    fn is_contact(&self) -> Result<bool> {
        let location = "contact.marker";
        match self.get_header().read(location)? {
            Some(&Value::Boolean(b)) => Ok(b),
            Some(_) => Err(CE::from_kind(CEK::HeaderTypeError("boolean", location))),
            None    => Ok(false)
        }
    }

    fn get_contact_data(&self) -> Result<ContactData> {
        let component = self
            .fs_file()
            .map_err(From::from)
            .and_then(util::read_to_string)
            .and_then(util::parse)?;

        Ok(ContactData(component))
    }

}

pub struct ContactData(Component);

impl ContactData {

    pub fn into_inner(self) -> Component {
        self.0
    }

}

impl Deref for ContactData {
    type Target = Component;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


