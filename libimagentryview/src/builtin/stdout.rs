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

use toml::encode_str;

use viewer::Viewer;
use result::Result;

pub struct StdoutViewer {
    view_header: bool,
    view_content: bool,
}

impl StdoutViewer {

    pub fn new(view_header: bool, view_content: bool) -> StdoutViewer {
        StdoutViewer {
            view_header: view_header,
            view_content: view_content,
        }
    }

}

impl Viewer for StdoutViewer {

    fn view_entry(&self, e: &Entry) -> Result<()> {
        if self.view_header {
            println!("{}", encode_str(e.get_header().header()));
        }

        if self.view_content {
            println!("{}", e.get_content());
        }

        Ok(())
    }

}
