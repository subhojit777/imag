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

}
