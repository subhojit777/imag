use libimagstore::store::EntryHeader;

use link::{Link, Links};
use result::Result;

pub fn get_link(header: &EntryHeader) -> Result<Link> {
    unimplemented!()
}

/// Set an external link in the header
///
/// Return the previous set link if there was any
pub fn set_link(header: &mut EntryHeader, l: Link) -> Option<Link> {
    unimplemented!()
}

