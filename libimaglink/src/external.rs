use libimagstore::store::EntryHeader;

use error::{LinkError, LinkErrorKind};
use link::{Link, Links};
use result::Result;

use toml::Value;
use toml::Table;

pub fn get_link(header: &EntryHeader) -> Result<Option<Link>> {
    let uri = header.read("imag.content.uri");

    if uri.is_err() {
        let kind = LinkErrorKind::EntryHeaderReadError;
        let lerr = LinkError::new(kind, Some(Box::new(uri.err().unwrap())));
        return Err(lerr);
    }
    let uri = uri.unwrap();

    match uri {
        Some(Value::String(s)) => Ok(Some(Link::new(s))),
        _ => Err(LinkError::new(LinkErrorKind::ExistingLinkTypeWrong, None)),
    }
}

/// Set an external link in the header
///
/// Return the previous set link if there was any
pub fn set_link(header: &mut EntryHeader, l: Link) -> Option<Link> {
    unimplemented!()
}

