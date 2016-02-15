use libimagstore::store::Entry;
use libimagstore::store::EntryHeader;
use libimagstore::store::Result as StoreResult;

use error::{LinkError, LinkErrorKind};
use link::{Link, Links};
use result::Result;

use toml::Value;

pub trait InternalLinker {

    /// Get the internal links from the implementor object
    fn get_internal_links(&self) -> Result<Links>;

    /// Set the internal links for the implementor object
    fn set_internal_links(&mut self, links: Links) -> Result<Links>;

    /// Add an internal link to the implementor object
    fn add_internal_link(&mut self, link: Link) -> Result<()>;

    /// Remove an internal link from the implementor object
    fn remove_internal_link(&mut self, link: Link) -> Result<()>;

}

impl InternalLinker for EntryHeader {

    fn get_internal_links(self: &EntryHeader) -> Result<Links> {
        process_rw_result(self.read("imag.links"))
    }

    /// Set the links in a header and return the old links, if any.
    fn set_internal_links(&mut self, links: Links) -> Result<Links> {
        let links : Vec<Link> = links.into();
        let links : Vec<Value> = links.into_iter().map(|link| Value::String(link.into())).collect();
        process_rw_result(self.set("imag.links", Value::Array(links)))
    }

    fn add_internal_link(&mut self, link: Link) -> Result<()> {
        self.get_internal_links()
            .and_then(|mut links| {
                links.add(link);
                self.set_internal_links(links).map(|_| ())
            })
    }

    fn remove_internal_link(&mut self, link: Link) -> Result<()> {
        self.get_internal_links()
            .and_then(|mut links| {
                links.remove(link);
                self.set_internal_links(links).map(|_| ())
            })
    }

}

impl InternalLinker for Entry {

    fn get_internal_links(&self) -> Result<Links> {
        self.get_header().get_internal_links()
    }

    fn set_internal_links(&mut self, links: Links) -> Result<Links> {
        self.get_header_mut().set_internal_links(links)
    }

    fn add_internal_link(&mut self, link: Link) -> Result<()> {
        self.get_header_mut().add_internal_link(link)
    }

    fn remove_internal_link(&mut self, link: Link) -> Result<()> {
        self.get_header_mut().remove_internal_link(link)
    }
}

fn process_rw_result(links: StoreResult<Option<Value>>) -> Result<Links> {
    if links.is_err() {
        let lerr  = LinkError::new(LinkErrorKind::EntryHeaderReadError,
                                   Some(Box::new(links.err().unwrap())));
        return Err(lerr);
    }
    let links = links.unwrap();

    if links.iter().any(|l| match l { &Value::String(_) => true, _ => false }) {
        return Err(LinkError::new(LinkErrorKind::ExistingLinkTypeWrong, None));
    }

    let links : Vec<Link> = links.into_iter()
        .map(|link| {
            match link {
                Value::String(s) => Link::new(s),
                _ => unreachable!(),
            }
        })
        .collect();

    Ok(Links::new(links))
}

