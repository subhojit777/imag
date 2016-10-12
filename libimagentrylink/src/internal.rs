//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::cmp::Ordering;

use libimagstore::storeid::StoreId;
use libimagstore::store::Entry;
use libimagstore::store::EntryHeader;
use libimagstore::store::Result as StoreResult;
use libimagerror::into::IntoError;

use error::LinkErrorKind as LEK;
use error::MapErrInto;
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
            new_links.push(link);
        }

        let new_links = try!(links_into_values(new_links)
                             .into_iter()
                             .fold(Ok(vec![]), |acc, elem| {
                                acc.and_then(move |mut v| {
                                    elem.map_err_into(LEK::InternalConversionError)
                                        .map(|e| {
                                            v.push(e);
                                            v
                                        })
                                })
                            }));
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

fn links_into_values(links: Vec<StoreId>) -> Vec<Result<Value>> {
    links
        .into_iter()
        .map(|s| s.without_base().to_str().map_err_into(LEK::InternalConversionError))
        .unique_by(|entry| {
            match entry {
                &Ok(ref e) => Some(e.clone()),
                &Err(_) => None,
            }
        })
        .map(|elem| elem.map(Value::String))
        .sorted_by(|a, b| {
            match (a, b) {
                (&Ok(Value::String(ref a)), &Ok(Value::String(ref b))) => Ord::cmp(a, b),
                (&Err(_), _) | (_, &Err(_)) => Ordering::Equal,
                _ => unreachable!()
            }
        })
}

fn rewrite_links(header: &mut EntryHeader, links: Vec<StoreId>) -> Result<()> {
    let links = try!(links_into_values(links)
                     .into_iter()
                     .fold(Ok(vec![]), |acc, elem| {
                        acc.and_then(move |mut v| {
                            elem.map_err_into(LEK::InternalConversionError)
                                .map(|e| {
                                    v.push(e);
                                    v
                                })
                        })
                     }));

    let process = header.set("imag.links", Value::Array(links));
    process_rw_result(process).map(|_| ())
}

/// When Linking A -> B, the specification wants us to link back B -> A.
/// This is a helper function which does this.
fn add_foreign_link(target: &mut Entry, from: StoreId) -> Result<()> {
    target.get_internal_links()
        .and_then(|mut links| {
            links.push(from);
            let links = try!(links_into_values(links)
                             .into_iter()
                             .fold(Ok(vec![]), |acc, elem| {
                                acc.and_then(move |mut v| {
                                    elem.map_err_into(LEK::InternalConversionError)
                                        .map(|e| {
                                            v.push(e);
                                            v
                                        })
                                })
                             }));
            process_rw_result(target.get_header_mut().set("imag.links", Value::Array(links)))
                .map(|_| ())
        })
}

fn process_rw_result(links: StoreResult<Option<Value>>) -> Result<Vec<Link>> {
    use std::path::PathBuf;

    let links = match links {
        Err(e) => {
            debug!("RW action on store failed. Generating LinkError");
            return Err(LEK::EntryHeaderReadError.into_error_with_cause(Box::new(e)))
        },
        Ok(None) => {
            debug!("We got no value from the header!");
            return Ok(vec![])
        },
        Ok(Some(Value::Array(l))) => l,
        Ok(Some(_)) => {
            debug!("We expected an Array for the links, but there was a non-Array!");
            return Err(LEK::ExistingLinkTypeWrong.into());
        }
    };

    if !links.iter().all(|l| is_match!(*l, Value::String(_))) {
        debug!("At least one of the Values which were expected in the Array of links is a non-String!");
        debug!("Generating LinkError");
        return Err(LEK::ExistingLinkTypeWrong.into());
    }

    let links : Vec<Link> = try!(links.into_iter()
        .map(|link| {
            match link {
                Value::String(s) => StoreId::new_baseless(PathBuf::from(s))
                    .map_err_into(LEK::StoreIdError),
                _ => unreachable!(),
            }
        })
        .collect());

    debug!("Ok, the RW action was successful, returning link vector now!");
    Ok(links)
}

