//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use libimagstore::store::Entry;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;

use error::Result;
use util;

/// Trait to be implemented on ::libimagstore::store::Entry
pub trait Contact {

    fn is_contact(&self) -> Result<bool>;

    // getting data

    fn get_contact_data(&self) -> Result<ContactData>;

    // More convenience functionality may follow

}

provide_kindflag_path!(pub IsContact, "contact.is_contact");

impl Contact for Entry {

    fn is_contact(&self) -> Result<bool> {
        self.is::<IsContact>().map_err(From::from)
    }

    fn get_contact_data(&self) -> Result<ContactData> {
        unimplemented!()
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


