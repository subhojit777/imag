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

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::process::Command;
use std::io::Error as IOError;

use tempfile::NamedTempFile;

pub fn edit_in_tmpfile_with_command(mut cmd: Command, s: &mut String) -> Result<bool, IOError> {
    let file      = try!(NamedTempFile::new());
    let file_path = file.path();
    let mut file  = try!(file.reopen());

    try!(file.write_all(&s.clone().into_bytes()[..]));
    try!(file.sync_data());

    cmd.arg(file_path)
        .status()
        .and_then(|status| {
            if status.success() {
                file.sync_data()
                    .and_then(|_| file.seek(SeekFrom::Start(0)))
                    .and_then(|_| {
                        let mut new_s = String::new();
                        let res = file.read_to_string(&mut new_s);
                        *s = new_s;
                        res
                    })
                    .map(|_| true)
            } else {
                Ok(false)
            }
        })
}

