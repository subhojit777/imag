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

//! Module for the MailIter
//!
//! MailIter is a iterator which takes an Iterator that yields `Ref` and yields itself
//! `Result<Mail>`, where `Err(_)` is returned if the Ref is not a Mail or parsing of the
//! referenced mail file failed.
//!

use mail::Mail;
use error::Result;

use libimagstore::store::FileLockEntry;

use std::marker::PhantomData;

pub struct MailIter<'a, I: Iterator<Item = FileLockEntry<'a>>> {
    _marker: PhantomData<I>,
    i: I,
}

impl<'a, I: Iterator<Item = FileLockEntry<'a>>> MailIter<'a, I> {

    pub fn new(i: I) -> MailIter<'a, I> {
        MailIter { _marker: PhantomData, i: i }
    }

}

impl<'a, I: Iterator<Item = FileLockEntry<'a>>> Iterator for MailIter<'a, I> {
    type Item = Result<Mail<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i.next().map(Mail::from_fle)
    }

}

