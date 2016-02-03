use libimagstore::store::EntryHeader;

use error::{LinkError, LinkErrorKind};
use link::{Link, Links};
use result::Result;

pub fn get_links(header: &EntryHeader) -> Result<Links> {
    unimplemented!()
}

pub fn set_links(header: &mut EntryHeader, links: Links) -> Result<Links> {
    unimplemented!()
}

pub fn add_link(header: &mut EntryHeader, link: Link) -> Result<()> {
    unimplemented!()
}

