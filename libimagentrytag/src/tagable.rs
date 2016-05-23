use std::ops::Deref;
use std::ops::DerefMut;

use itertools::Itertools;

use libimagstore::store::{Entry, EntryHeader, FileLockEntry};

use error::{TagError, TagErrorKind};
use result::Result;
use tag::{Tag, TagSlice};
use util::is_tag;

use toml::Value;

pub trait Tagable {

    fn get_tags(&self) -> Result<Vec<Tag>>;
    fn set_tags(&mut self, ts: &[Tag]) -> Result<()>;

    fn add_tag(&mut self, t: Tag) -> Result<()>;
    fn remove_tag(&mut self, t: Tag) -> Result<()>;

    fn has_tag(&self, t: TagSlice) -> Result<bool>;
    fn has_tags(&self, ts: &[Tag]) -> Result<bool>;

}

impl Tagable for EntryHeader {

    fn get_tags(&self) -> Result<Vec<Tag>> {
        let tags = self.read("imag.tags");
        if tags.is_err() {
            let kind = TagErrorKind::HeaderReadError;
            return Err(TagError::new(kind, Some(Box::new(tags.unwrap_err()))));
        }
        let tags = tags.unwrap();

        match tags {
            Some(Value::Array(tags)) => {
                if !tags.iter().all(|t| is_match!(*t, Value::String(_))) {
                    return Err(TagError::new(TagErrorKind::TagTypeError, None));
                }
                if tags.iter().any(|t| match *t {
                    Value::String(ref s) => !is_tag(s),
                    _ => unreachable!()})
                {
                    return Err(TagError::new(TagErrorKind::NotATag, None));
                }

                Ok(tags.iter()
                    .cloned()
                    .map(|t| {
                        match t {
                           Value::String(s) => s,
                           _ => unreachable!(),
                        }
                    })
                    .collect())
            },
            None => Ok(vec![]),
            _ => Err(TagError::new(TagErrorKind::TagTypeError, None)),
        }
    }

    fn set_tags(&mut self, ts: &[Tag]) -> Result<()> {
        if ts.iter().any(|tag| !is_tag(tag)) {
            debug!("Not a tag: '{}'", ts.iter().filter(|t| !is_tag(t)).next().unwrap());
            return Err(TagError::new(TagErrorKind::NotATag, None));
        }

        let a = ts.iter().unique().map(|t| Value::String(t.clone())).collect();
        self.set("imag.tags", Value::Array(a))
            .map(|_| ())
            .map_err(|e| TagError::new(TagErrorKind::HeaderWriteError, Some(Box::new(e))))
    }

    fn add_tag(&mut self, t: Tag) -> Result<()> {
        if !is_tag(&t) {
            debug!("Not a tag: '{}'", t);
            return Err(TagError::new(TagErrorKind::NotATag, None));
        }

        self.get_tags()
            .map(|mut tags| {
                tags.push(t);
                self.set_tags(&tags.into_iter().unique().collect::<Vec<_>>()[..])
            })
            .map(|_| ())
    }

    fn remove_tag(&mut self, t: Tag) -> Result<()> {
        if !is_tag(&t) {
            debug!("Not a tag: '{}'", t);
            return Err(TagError::new(TagErrorKind::NotATag, None));
        }

        self.get_tags()
            .map(|mut tags| {
                tags.retain(|tag| tag.clone() != t);
                self.set_tags(&tags[..])
            })
            .map(|_| ())
    }

    fn has_tag(&self, t: TagSlice) -> Result<bool> {
        let tags = self.read("imag.tags");
        if tags.is_err() {
            let kind = TagErrorKind::HeaderReadError;
            return Err(TagError::new(kind, Some(Box::new(tags.unwrap_err()))));
        }
        let tags = tags.unwrap();

        if !tags.iter().all(|t| is_match!(*t, Value::String(_))) {
            return Err(TagError::new(TagErrorKind::TagTypeError, None));
        }

        Ok(tags
           .iter()
           .any(|tag| {
               match *tag {
                   Value::String(ref s) => { s == t },
                   _ => unreachable!()
               }
           }))
    }

    fn has_tags(&self, tags: &[Tag]) -> Result<bool> {
        let mut result = true;
        for tag in tags {
            let check = self.has_tag(tag);
            if check.is_err() {
                return Err(check.unwrap_err());
            }
            let check = check.unwrap();

            result = result && check;
        }

        Ok(result)
    }

}

impl Tagable for Entry {

    fn get_tags(&self) -> Result<Vec<Tag>> {
        self.get_header().get_tags()
    }

    fn set_tags(&mut self, ts: &[Tag]) -> Result<()> {
        self.get_header_mut().set_tags(ts)
    }

    fn add_tag(&mut self, t: Tag) -> Result<()> {
        self.get_header_mut().add_tag(t)
    }

    fn remove_tag(&mut self, t: Tag) -> Result<()> {
        self.get_header_mut().remove_tag(t)
    }

    fn has_tag(&self, t: TagSlice) -> Result<bool> {
        self.get_header().has_tag(t)
    }

    fn has_tags(&self, ts: &[Tag]) -> Result<bool> {
        self.get_header().has_tags(ts)
    }

}

impl<'a> Tagable for FileLockEntry<'a> {

    fn get_tags(&self) -> Result<Vec<Tag>> {
        self.deref().get_tags()
    }

    fn set_tags(&mut self, ts: &[Tag]) -> Result<()> {
        self.deref_mut().set_tags(ts)
    }

    fn add_tag(&mut self, t: Tag) -> Result<()> {
        self.deref_mut().add_tag(t)
    }

    fn remove_tag(&mut self, t: Tag) -> Result<()> {
        self.deref_mut().remove_tag(t)
    }

    fn has_tag(&self, t: TagSlice) -> Result<bool> {
        self.deref().has_tag(t)
    }

    fn has_tags(&self, ts: &[Tag]) -> Result<bool> {
        self.deref().has_tags(ts)
    }

}

