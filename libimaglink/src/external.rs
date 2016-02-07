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
pub fn set_link(header: &mut EntryHeader, l: Link) -> Result<Option<Link>> {
    let old_link = header.set("imag.content.uri", Value::String(l.into()));

    if old_link.is_err() {
        let kind = LinkErrorKind::EntryHeaderWriteError;
        let lerr = LinkError::new(kind, Some(Box::new(old_link.err().unwrap())));
        return Err(lerr);
    }
    let old_link = old_link.unwrap();

    if old_link.is_none() {
        return Ok(None);
    }

    match old_link.unwrap() {
        Value::String(s) => Ok(Some(Link::new(s))),

        // We don't do anything in this case and be glad we corrected the type error with this set()
        _ => Ok(None),
    }
}

