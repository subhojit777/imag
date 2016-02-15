use libimagstore::store::EntryHeader;
use libimagstore::store::Result as StoreResult;

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
    get_links(header).and_then(|mut links| {
        links.add(link);
        set_links(header, links).map(|_| ())
    })
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

