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

