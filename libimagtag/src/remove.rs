use toml::Value;

use libimagstore::store::Entry;

use result::Result;
use tag::Tag;
use error::{TagError, TagErrorKind};

pub fn remove_tag(e: &mut Entry, t: &Tag) -> Result<()> {
}
