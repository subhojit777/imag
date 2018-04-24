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

use std::io::Write;

use libimagstore::store::Entry;
use libimagrt::runtime::Runtime;

use mdcat::{ResourceAccess, Terminal, TerminalSize};
use pulldown_cmark::Parser;
use syntect::parsing::SyntaxSet;
use mdcat;

use viewer::Viewer;
use error::Result;

pub struct MarkdownViewer<'a> {
    rt:                 &'a Runtime<'a>,
    resource_access:    ResourceAccess,
    terminal:           Terminal,
    termsize:           TerminalSize,
}

impl<'a> MarkdownViewer<'a> {
    pub fn new(rt: &'a Runtime) -> Self {
        MarkdownViewer {
            rt,
            resource_access: ResourceAccess::LocalOnly,
            terminal:        Terminal::detect(),
            termsize:        TerminalSize::detect().unwrap_or(TerminalSize {
                width: 80,
                height: 20,
            }),
        }
    }
}

impl<'a> Viewer for MarkdownViewer<'a> {
    fn view_entry<W>(&self, e: &Entry, sink: &mut W) -> Result<()>
        where W: Write
    {
        let parser          = Parser::new(e.get_content());
        let base_dir        = self.rt.rtp();
        let syntax_set      = SyntaxSet::load_defaults_newlines();

        mdcat::push_tty(sink,
                        self.terminal.clone(),
                        self.termsize.clone(),
                        parser,
                        base_dir,
                        self.resource_access.clone(),
                        syntax_set)
        .map_err(|e| e.compat())
        .map_err(::error::ViewError::from)
    }
}

