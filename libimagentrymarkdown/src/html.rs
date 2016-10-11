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

use hoedown::{Markdown, Html as MdHtml};
use hoedown::renderer::html::Flags as HtmlFlags;
use hoedown::renderer::Render;

use result::Result;
use error::MarkdownErrorKind;
use libimagerror::into::IntoError;

pub type HTML = String;

pub fn to_html(buffer: &str) -> Result<HTML> {
    let md = Markdown::new(buffer);
    let mut html = MdHtml::new(HtmlFlags::empty(), 0);
    html.render(&md)
        .to_str()
        .map(String::from)
        .map_err(Box::new)
        .map_err(|e| MarkdownErrorKind::MarkdownRenderError.into_error_with_cause(e))
}

pub mod iter {
    use result::Result;
    use libimagstore::store::Entry;
    use super::HTML;
    use super::to_html;

    pub struct ToHtmlIterator<I: Iterator<Item = Entry>> {
        i: I
    }

    impl<I: Iterator<Item = Entry>> ToHtmlIterator<I> {

        fn new(i: I) -> ToHtmlIterator<I> {
            ToHtmlIterator { i: i }
        }

    }

    impl<I: Iterator<Item = Entry>> Iterator for ToHtmlIterator<I> {
        type Item = Result<HTML>;

        fn next(&mut self) -> Option<Self::Item> {
            self.i.next().map(|entry| to_html(&entry.get_content()[..]))
        }

    }

    impl<I: Iterator<Item = Entry>> From<I> for ToHtmlIterator<I> {

        fn from(obj: I) -> ToHtmlIterator<I> {
            ToHtmlIterator::new(obj)
        }

    }


    /// Iterate over `(Entry, Result<HTML>)` tuples
    pub struct WithHtmlIterator<I: Iterator<Item = Entry>> {
        i: I
    }

    impl<I: Iterator<Item = Entry>> WithHtmlIterator<I> {

        fn new(i: I) -> WithHtmlIterator<I> {
            WithHtmlIterator { i: i }
        }

    }

    impl<I: Iterator<Item = Entry>> Iterator for WithHtmlIterator<I> {
        type Item = (Entry, Result<HTML>);

        fn next(&mut self) -> Option<Self::Item> {
            self.i.next().map(|entry| {
                let html = to_html(&entry.get_content()[..]);
                (entry, html)
            })
        }

    }

}
