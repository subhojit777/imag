use toml::Value;

use libimagstore::store::Entry;

use result::Result;
use tag::Tag;
use error::{TagError, TagErrorKind};

pub fn remove_tag(e: &mut Entry, t: &Tag) -> Result<()> {
    let tags = e.get_header().read("imag.tags");
    if tags.is_err() {
        let kind = TagErrorKind::HeaderReadError;
        return Err(TagError::new(kind, Some(Box::new(tags.err().unwrap()))));
    }
    let tags = tags.unwrap();

    if !tags.iter().all(|t| match t { &Value::String(_) => true, _ => false }) {
        return Err(TagError::new(TagErrorKind::TagTypeError, None));
    }

    if tags.is_none() {
        return Ok(());
    }
    let tags = tags.unwrap();

    if !match tags { Value::Array(_) => true, _ => false } {
        return Err(TagError::new(TagErrorKind::TagTypeError, None));
    }

    match tags {
        Value::Array(tag_array) => {
            let new_tags = tag_array.iter()
                .map(|tag| {
                    match tag {
                        &Value::String(ref s) => s,
                        _ => unreachable!(),
                    }
                })
                .filter(|tag| tag.clone() != t)
                .map(|tag| Value::String(t.clone()))
                .collect();

            e.get_header_mut()
                .set("imag.tags", Value::Array(new_tags))
                .map_err(|e| TagError::new(TagErrorKind::TagTypeError, Some(Box::new(e))))
                .map(|_| ())
        },

        _ => unreachable!(),
    }
}
