use std::convert::Into;

use libimagstore::store::Entry;

pub struct Link {
    link: String
}

impl Link {

    pub fn new(s: String) -> Link {
        Link { link: s }
    }

}

pub struct Links {
    links: Vec<Link>,
}

impl Links {
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

