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

use std::fmt::{Display, Formatter, Error};

/// Utility type to specify which kind of store action is running
#[derive(Clone, Debug)]
pub enum StoreAction {
    Create,
    Retrieve,
    Update,
    Delete,
}

impl StoreAction {

    pub fn uppercase(&self) -> &str {
        match *self {
            StoreAction::Create   => "CREATE",
            StoreAction::Retrieve => "RETRIEVE",
            StoreAction::Update   => "UPDATE",
            StoreAction::Delete   => "DELETE",
        }
    }

    pub fn as_commit_message(&self) -> &str {
        match *self {
            StoreAction::Create   => "Create",
            StoreAction::Retrieve => "Retrieve",
            StoreAction::Update   => "Update",
            StoreAction::Delete   => "Delete",
        }
    }
}

impl Display for StoreAction {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "StoreAction: {}",
                match *self {
                    StoreAction::Create   => "create",
                    StoreAction::Retrieve => "retrieve",
                    StoreAction::Update   => "update",
                    StoreAction::Delete   => "delete",
                })
    }

}

