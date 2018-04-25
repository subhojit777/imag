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

use std::path::PathBuf;

use chrono::NaiveDateTime;

use libimagutil::date::datetime_to_string;
use libimagstore::storeid::StoreId;

use error::Result;

pub fn mk_group_path(name: &String) -> PathBuf {
    PathBuf::from(format!("groups/{}/group", name))
}

pub fn mk_session_path(groupname: &String, datetime: &NaiveDateTime) -> PathBuf {
    let datetime = datetime_to_string(datetime);
    PathBuf::from(format!("sessions/{group}/{dt}", group = groupname, dt = datetime))
}

pub fn mk_card_path(groupname: &String, question: &String) -> Result<PathBuf> {
    // let question = hashof(question); TODO: Hash me
    Ok(PathBuf::from(format!("groups/{group}/cards/{id}", group = groupname, id = question)))
}


pub trait IsGroupId {
    fn is_group_id(&self) -> bool;
}

impl IsGroupId for StoreId {
    fn is_group_id(&self) -> bool {
        trace!("Checking whether '{}' is a group id", self);
        self.is_in_collection(&["groups"]) && self.local().ends_with("group")
    }
}


pub trait IsSessionId {
    fn is_session_id(&self) -> bool;
}

impl IsSessionId for StoreId {
    fn is_session_id(&self) -> bool {
        trace!("Checking whether '{}' is a session id", self);
        self.is_in_collection(&["sessions"])
    }
}


pub trait IsCardId {
    fn is_card_id(&self) -> bool;
}

impl IsCardId for StoreId {
    fn is_card_id(&self) -> bool {
        trace!("Checking whether '{}' is a card id", self);

        self.is_in_collection(&["groups"]) &&
            self.local().to_str().map(|s| s.contains("cards")).unwrap_or(false)
    }
}

