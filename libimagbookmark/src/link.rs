use std::ops::{Deref, DerefMut};

use result::Result;

use url::Url;

#[derive(Debug, Clone)]
pub struct Link(String);

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

