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

use std::ops::{Deref, DerefMut};

use result::Result;

use url::Url;

#[derive(Debug, Clone)]
pub struct Link(String);

impl From<String> for Link {

    fn from(s: String) -> Link {
        Link(s)
    }

}

impl<'a> From<&'a str> for Link {

    fn from(s: &'a str) -> Link {
        Link(String::from(s))
    }

}

impl Deref for Link {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }

}

impl DerefMut for Link {

    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }

}

pub trait IntoUrl {
    fn into_url(self) -> Result<Url>;
}

impl IntoUrl for Link {

    fn into_url(self) -> Result<Url> {
        use error::BookmarkErrorKind as BEK;
        use error::MapErrInto;

        Url::parse(&self[..]).map_err_into(BEK::LinkParsingError)
    }

}

