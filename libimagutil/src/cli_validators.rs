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

//! Functions to be used for clap::Arg::validator()
//! to validate arguments

use std::path::PathBuf;
use boolinator::Boolinator;

pub fn is_existing_path(s: String) -> Result<(), String> {
    PathBuf::from(s.clone()).exists().as_result((), format!("Not a File or Directory: {}", s))
}

pub fn is_file(s: String) -> Result<(), String> {
    PathBuf::from(s.clone()).is_file().as_result((), format!("Not a File: {}", s))
}

pub fn is_directory(s: String) -> Result<(), String> {
    PathBuf::from(s.clone()).is_dir().as_result((), format!("Not a Directory: {}", s))
}

pub fn is_integer(s: String) -> Result<(), String> {
    use std::str::FromStr;

    let i : Result<i64, _> = FromStr::from_str(&s);
    i.map(|_| ()).map_err(|_| format!("Not an integer: {}", s))
}

pub fn is_url(s: String) -> Result<(), String> {
    use url::Url;
    Url::parse(&s).map(|_| ()).map_err(|_| format!("Not a URL: {}", s))
}

