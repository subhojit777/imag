use libimagstore::storeid::StoreId;
use libimagstore::store::Entry;
use libimagstore::store::EntryHeader;
use libimagstore::store::Result as StoreResult;

use error::{LinkError, LinkErrorKind};
use result::Result;

use toml::Value;
use itertools::Itertools;

pub type Link = StoreId;

pub trait InternalLinker {

    /// Get the internal links from the implementor object
    fn get_internal_links(&self) -> Result<Vec<Link>>;

    /// Set the internal links for the implementor object
    fn set_internal_links(&mut self, links: Vec<&mut Entry>) -> Result<Vec<Link>>;

    /// Add an internal link to the implementor object
    fn add_internal_link(&mut self, link: &mut Entry) -> Result<()>;

    /// Remove an internal link from the implementor object
    fn remove_internal_link(&mut self, link: &mut Entry) -> Result<()>;

}

impl InternalLinker for Entry {

    fn get_internal_links(&self) -> Result<Vec<Link>> {
        process_rw_result(self.get_header().read("imag.links"))
    }

    /// Set the links in a header and return the old links, if any.
    fn set_internal_links(&mut self, links: Vec<&mut Entry>) -> Result<Vec<Link>> {
        let self_location = self.get_location().clone();
        let mut new_links = vec![];

        for link in links {
            if let Err(e) = add_foreign_link(link, self_location.clone()) {
                return Err(e);
            }
            let link = link.get_location().clone();
            let link = link.to_str();
            if link.is_none() {
                return Err(LinkError::new(LinkErrorKind::InternalConversionError, None));
            }
            let link = link.unwrap();

            new_links.push(String::from(link));
        }

        let new_links = new_links.into_iter().unique().map(|s| Value::String(s)).collect();
        process_rw_result(self.get_header_mut().set("imag.links", Value::Array(new_links)))
    }

    fn add_internal_link(&mut self, link: &mut Entry) -> Result<()> {
        let new_link = link.get_location().clone();

        add_foreign_link(link, self.get_location().clone())
            .and_then(|_| {
                self.get_internal_links()
                    .and_then(|mut links| {
                        links.push(new_link);
                        rewrite_links(self.get_header_mut(), links)
                    })
            })
    }

    fn remove_internal_link(&mut self, link: &mut Entry) -> Result<()> {
        let own_loc   = link.get_location().clone();
        let other_loc = link.get_location().clone();

        link.get_internal_links()
            .and_then(|links| {
                let links = links.into_iter().filter(|l| l.clone() != own_loc).collect();
                rewrite_links(self.get_header_mut(), links)
            })
            .and_then(|_| {
                self.get_internal_links()
                    .and_then(|links| {
                        let links = links.into_iter().filter(|l| l.clone() != other_loc).collect();
                        rewrite_links(link.get_header_mut(), links)
                    })
            })
    }

}

fn rewrite_links(header: &mut EntryHeader, links: Vec<StoreId>) -> Result<()> {
    let links : Vec<Option<Value>> = links
        .into_iter()
        .map(|s| s.to_str().map(|s| String::from(s)))
        .unique()
        .map(|elem| elem.map(|s| Value::String(s)))
        .collect();

    if links.iter().any(|o| o.is_none()) {
        // if any type convert failed we fail as well
        Err(LinkError::new(LinkErrorKind::InternalConversionError, None))
    } else {
        // I know it is ugly
        let links = links.into_iter().map(|opt| opt.unwrap()).collect();
        let process = header.set("imag.links", Value::Array(links));
        process_rw_result(process).map(|_| ())
    }
}

/// When Linking A -> B, the specification wants us to link back B -> A.
/// This is a helper function which does this.
fn add_foreign_link(target: &mut Entry, from: StoreId) -> Result<()> {
    target.get_internal_links()
        .and_then(|mut links| {
            links.push(from);
            let links : Vec<Option<Value>> = links
                .into_iter()
                .map(|s| {
                    match s.to_str() {
                        Some(s) => Some(String::from(s)),
                        _ => None
                    }
                })
                .unique()
                .map(|elem| elem.map(|s| Value::String(s)))
                .collect();
            if links.iter().any(|o| o.is_none()) {
                Err(LinkError::new(LinkErrorKind::InternalConversionError, None))
            } else {
                let links = links.into_iter().map(|opt| opt.unwrap()).collect();
                process_rw_result(target.get_header_mut().set("imag.links", Value::Array(links)))
                    .map(|_| ())
            }
        })
}

fn process_rw_result(links: StoreResult<Option<Value>>) -> Result<Vec<Link>> {
    if links.is_err() {
        debug!("RW action on store failed. Generating LinkError");
        let lerr  = LinkError::new(LinkErrorKind::EntryHeaderReadError,
                                   Some(Box::new(links.err().unwrap())));
        return Err(lerr);
    }
    let links = links.unwrap();

    if links.is_none() {
        debug!("We got no value from the header!");
        return Ok(vec![])
    }
    let links = links.unwrap();

    let links = {
        match links {
            Value::Array(a) => a,
            _ => {
                debug!("We expected an Array for the links, but there was a non-Array!");
                return Err(LinkError::new(LinkErrorKind::ExistingLinkTypeWrong, None));
            },
        }
    };

    if !links.iter().all(|l| match l { &Value::String(_) => true, _ => false }) {
        debug!("At least one of the Values which were expected in the Array of links is a non-String!");
        debug!("Generating LinkError");
        return Err(LinkError::new(LinkErrorKind::ExistingLinkTypeWrong, None));
    }

    let links : Vec<Link> = links.into_iter()
        .map(|link| {
            match link {
                Value::String(s) => StoreId::from(s),
                _ => unreachable!(),
            }
        })
        .collect();

    debug!("Ok, the RW action was successful, returning link vector now!");
    Ok(links)
}

