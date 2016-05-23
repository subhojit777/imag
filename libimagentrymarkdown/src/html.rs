use result::Result;
use error::MarkdownErrorKind;

pub type HTML = String;

pub fn to_html(buffer: &str) -> Result<HTML> {
    unimplemented!()
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

}
