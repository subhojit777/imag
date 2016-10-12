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

use libimagstore::store::Entry;
use libimagrt::runtime::Runtime;
use libimagentryedit::edit::edit_in_tmpfile;

use viewer::Viewer;
use result::Result;
use error::ViewErrorKind as VEK;
use error::ViewError as VE;

pub struct EditorView<'a>(&'a Runtime<'a>);

impl<'a> EditorView<'a> {
    pub fn new(rt: &'a Runtime) -> EditorView<'a> {
        EditorView(rt)
    }
}

impl<'a> Viewer for EditorView<'a> {
    fn view_entry(&self, e: &Entry) -> Result<()> {
        let mut entry = e.to_str().clone().to_string();
        edit_in_tmpfile(self.0, &mut entry)
            .map_err(|e| VE::new(VEK::ViewError, Some(Box::new(e))))
    }
}

