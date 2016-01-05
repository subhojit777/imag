use std::convert::{From, Into};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::hash::Hash;
use uuid::Uuid;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
/**
 * FileHash type
 *
 * Simple abstraction over String by now.
 */
pub struct FileHash {
    hash: String,
}

impl From<String> for FileHash {

    fn from(s: String) -> FileHash {
        FileHash { hash: s }
    }

}

impl<'a> From<&'a String> for FileHash {

    fn from(s: &'a String) -> FileHash {
        FileHash::from(s.clone())
    }

}

impl From<Uuid> for FileHash {

    fn from(u: Uuid) -> FileHash {
        FileHash::from(u.to_hyphenated_string())
    }

}

impl<'a> From<&'a str> for FileHash {

    fn from(s: &str) -> FileHash {
        FileHash::from(String::from(s))
    }

}

impl Into<String> for FileHash {

    fn into(self) -> String {
        self.hash
    }
}

impl Display for FileHash {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        try!(write!(fmt, "{}", self.hash));
        Ok(())
    }

}
