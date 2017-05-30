//! Module for the MailIter
//!
//! MailIter is a iterator which takes an Iterator that yields `Ref` and yields itself
//! `Result<Mail>`, where `Err(_)` is returned if the Ref is not a Mail or parsing of the
//! referenced mail file failed.
//!

use mail::Mail;
use result::Result;

use libimagref::reference::Ref;

use std::marker::PhantomData;

struct MailIter<'a, I: 'a + Iterator<Item = Ref<'a>>> {
    _marker: PhantomData<&'a I>,
    i: I,
}

impl<'a, I: Iterator<Item = Ref<'a>>> MailIter<'a, I> {

    pub fn new(i: I) -> MailIter<'a, I> {
        MailIter { _marker: PhantomData, i: i }
    }

}

impl<'a, I: Iterator<Item = Ref<'a>>> Iterator for MailIter<'a, I> {

    type Item = Result<Mail<'a>>;

    fn next(&mut self) -> Option<Result<Mail<'a>>> {
        self.i.next().map(Mail::from_ref)
    }

}

