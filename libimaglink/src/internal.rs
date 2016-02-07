use libimagstore::store::EntryHeader;

use error::{LinkError, LinkErrorKind};
use link::{Link, Links};
use result::Result;

use toml::Value;

pub fn get_links(header: &EntryHeader) -> Result<Links> {
    process_rw_result(header.read("imag.links"))
}

/// Set the links in a header and return the old links, if any.
pub fn set_links(header: &mut EntryHeader, links: Links) -> Result<Links> {
    let links : Vec<Link> = links.into();
    let links : Vec<Value> = links.into_iter().map(|link| Value::String(link.into())).collect();
    process_rw_result(header.set("imag.links", Value::Array(links)))
}

pub fn add_link(header: &mut EntryHeader, link: Link) -> Result<()> {
    unimplemented!()
}

