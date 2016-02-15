use std::convert::Into;

use libimagstore::store::Entry;

#[derive(PartialOrd, Ord, Eq, PartialEq, Clone, Debug)]
pub struct Link {
    link: String
}

impl Link {

    pub fn new(s: String) -> Link {
        Link { link: s }
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

