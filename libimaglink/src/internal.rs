use libimagstore::store::EntryHeader;

use error::{LinkError, LinkErrorKind};
use link::{Link, Links};
use result::Result;

use toml::Value;

pub fn get_links(header: &EntryHeader) -> Result<Links> {
    process_rw_result(header.read("imag.links"))
}

pub fn set_links(header: &mut EntryHeader, links: Links) -> Result<Links> {
    unimplemented!()
}

pub fn add_link(header: &mut EntryHeader, link: Link) -> Result<()> {
    unimplemented!()
}

