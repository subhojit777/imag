use std::convert::Into;

use libimagstore::store::Entry;
use libimagstore::store::EntryHeader;

use error::{LinkError, LinkErrorKind};
use result::Result;

use toml::Value;
use toml::Table;
use url::Url;

#[derive(PartialOrd, Ord, Eq, PartialEq, Clone, Debug)]
pub struct Link {
    link: String
}

impl Link {

    pub fn new(s: String) -> Link {
        Link { link: s }
    }

    pub fn is_valid(&self) -> bool {
        Url::parse(&self.link[..]).is_ok()
    }

}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Links {
    links: Vec<Link>,
}

impl Links {

    pub fn new(s: Vec<Link>) -> Links {
        Links { links: s }
    }

    pub fn add(&mut self, l: Link) {
        self.links.push(l);
    }

    pub fn remove(&mut self, l: Link) {
        self.links.retain(|link| l != link.clone());
    }

    pub fn all_valid(&self) -> bool {
        self.links.iter().all(|l| l.is_valid())
    }

}

impl Into<String> for Link {

    fn into(self) -> String {
        self.link
    }

}

impl Into<Vec<Link>> for Links {

    fn into(self) -> Vec<Link> {
        self.links
    }

}

pub trait ExternalLinker {

    /// get the external link from the implementor object
    fn get_external_link(&self) -> Result<Option<Link>>;

    /// set the external link for the implementor object and return the current link from the entry,
    /// if any.
    fn set_external_link(&mut self, l: Link) -> Result<Option<Link>>;
}

impl ExternalLinker for EntryHeader {

    fn get_external_link(&self) -> Result<Option<Link>> {
        let uri = self.read("imag.content.uri");

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
    fn set_external_link(&mut self, l: Link) -> Result<Option<Link>> {
        if !l.is_valid() {
            return Err(LinkError::new(LinkErrorKind::InvalidUri, None));
        }

        let old_link = self.set("imag.content.uri", Value::String(l.into()));

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

}

impl ExternalLinker for Entry {

    fn get_external_link(&self) -> Result<Option<Link>> {
        self.get_header().get_external_link()
    }

    fn set_external_link(&mut self, l: Link) -> Result<Option<Link>> {
        self.get_header_mut().set_external_link(l)
    }

}
