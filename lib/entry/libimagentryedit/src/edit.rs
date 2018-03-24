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

use libimagrt::runtime::Runtime;
use libimagstore::store::Entry;

use error::Result;
use error::EditErrorKind;
use error::EditError as EE;
use error::ResultExt;

pub trait Edit {
    fn edit_content(&mut self, rt: &Runtime) -> Result<()>;
}

impl Edit for String {

    fn edit_content(&mut self, rt: &Runtime) -> Result<()> {
        edit_in_tmpfile(rt, self).map(|_| ())
    }

}

impl Edit for Entry {

    fn edit_content(&mut self, rt: &Runtime) -> Result<()> {
        edit_in_tmpfile(rt, self.get_content_mut())
            .map(|_| ())
    }

}

pub fn edit_in_tmpfile(rt: &Runtime, s: &mut String) -> Result<()> {
    use libimagutil::edit::edit_in_tmpfile_with_command;

    let editor = rt
        .editor()
        .chain_err(|| EditErrorKind::NoEditor)?
        .ok_or_else(|| EE::from_kind(EditErrorKind::NoEditor))?;

    edit_in_tmpfile_with_command(editor, s)
        .chain_err(|| EditErrorKind::IOError)
        .and_then(|worked| {
            if !worked {
                Err(EditErrorKind::ProcessExitFailure.into())
            } else {
                Ok(())
            }
        })
}

