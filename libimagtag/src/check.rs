use toml::Value;

use libimagstore::store::{Entry, EntryHeader};

use result::Result;
use tag::Tag;
use error::{TagError, TagErrorKind};

pub fn has_tag(e: &Entry, t: &Tag) -> Result<bool> {
    header_has_tag(e.get_header(), t)
}

pub fn has_tags(e: &Entry, tags: &Vec<Tag>) -> Result<bool> {
    let hdr        = e.get_header();
    let mut result = true;

    for tag in tags {
        let check = header_has_tag(hdr, tag);
        if check.is_err() {
            return Err(check.err().unwrap());
        }
        let check = check.unwrap();

        result = result && check;
    }

    Ok(result)
}

fn header_has_tag(head: &EntryHeader, t: &Tag) -> Result<bool> {
    let tags = head.read("imag.tags");
    if tags.is_err() {
        let kind = TagErrorKind::HeaderReadError;
        return Err(TagError::new(kind, Some(Box::new(tags.err().unwrap()))));
    }
    let tags = tags.unwrap();

    if !tags.iter().all(|t| match t { &Value::String(_) => true, _ => false }) {
        return Err(TagError::new(TagErrorKind::TagTypeError, None));
    }

    Ok(tags
       .iter()
       .any(|tag| {
           match tag {
               &Value::String(ref s) => { s == t },
               _ => unreachable!()
           }
       }))
}

