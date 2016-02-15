use libimagstore::store::{Entry, EntryHeader};

use error::{TagError, TagErrorKind};
use result::Result;
use tag::Tag;

use toml::Value;

pub trait Tagable {

    fn get_tags(&self) -> Result<Vec<Tag>>;
    fn set_tags(&mut self, ts: Vec<Tag>) -> Result<()>;

    fn add_tag(&mut self, t: Tag) -> Result<()>;
    fn remove_tag(&mut self, t: Tag) -> Result<()>;

    fn has_tag(&self, t: &Tag) -> Result<bool>;
    fn has_tags(&self, ts: &Vec<Tag>) -> Result<bool>;

}

impl Tagable for EntryHeader {

    fn get_tags(&self) -> Result<Vec<Tag>> {
        let tags = self.read("imag.tags");
        if tags.is_err() {
            let kind = TagErrorKind::HeaderReadError;
            return Err(TagError::new(kind, Some(Box::new(tags.err().unwrap()))));
        }
        let tags = tags.unwrap();

        match tags {
            Some(Value::Array(tags)) => {
                if !tags.iter().all(|t| match t { &Value::String(_) => true, _ => false }) {
                    return Err(TagError::new(TagErrorKind::TagTypeError, None));
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

    fn set_tags(&mut self, ts: Vec<Tag>) -> Result<()> {
        let a = ts.iter().map(|t| Value::String(t.clone())).collect();
        self.set("imag.tags", Value::Array(a))
            .map(|_| ())
            .map_err(|e| TagError::new(TagErrorKind::HeaderWriteError, Some(Box::new(e))))
    }

    fn add_tag(&mut self, t: Tag) -> Result<()> {
        let tags = self.read("imag.tags");
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
                let mut new_tags = tag_array.clone();
                new_tags.push(Value::String(t.clone()));

                self.set("imag.tags", Value::Array(new_tags))
                    .map_err(|e| TagError::new(TagErrorKind::TagTypeError, Some(Box::new(e))))
                    .map(|_| ())
            },

            _ => unreachable!(),
        }
    }

    fn remove_tag(&mut self, t: Tag) -> Result<()> {
        let tags = self.read("imag.tags");
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
                let mut tag_array = tag_array.clone();
                tag_array.retain(|tag| {
                    match tag {
                        &Value::String(ref s) => s.clone() != t,
                        _ => unreachable!(),
                    }
                });

                self.set("imag.tags", Value::Array(tag_array))
                    .map_err(|e| TagError::new(TagErrorKind::TagTypeError, Some(Box::new(e))))
                    .map(|_| ())
            },

            _ => unreachable!(),
        }
    }

    fn has_tag(&self, t: &Tag) -> Result<bool> {
        let tags = self.read("imag.tags");
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

    fn has_tags(&self, tags: &Vec<Tag>) -> Result<bool> {
        let mut result = true;
        for tag in tags {
            let check = self.has_tag(tag);
            if check.is_err() {
                return Err(check.err().unwrap());
            }
            let check = check.unwrap();

            result = result && check;
        }

        Ok(result)
    }

}

