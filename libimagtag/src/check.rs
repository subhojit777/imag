use toml::Value;

use libimagstore::store::{Entry, EntryHeader};

use result::Result;
use tag::Tag;
use error::{TagError, TagErrorKind};

pub fn has_tag(e: &Entry, t: &Tag) -> Result<bool> {
}

pub fn has_tags(e: &Entry, tags: &Vec<Tag>) -> Result<bool> {
}

