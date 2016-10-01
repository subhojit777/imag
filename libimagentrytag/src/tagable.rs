//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::ops::Deref;
use std::ops::DerefMut;

use itertools::Itertools;

use libimagstore::store::{Entry, EntryHeader, FileLockEntry};
use libimagerror::into::IntoError;

use error::TagErrorKind;
use error::MapErrInto;
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
        let tags = try!(self.read("imag.tags").map_err_into(TagErrorKind::HeaderReadError));

        match tags {
            Some(Value::Array(tags)) => {
                if !tags.iter().all(|t| is_match!(*t, Value::String(_))) {
                    return Err(TagErrorKind::TagTypeError.into());
                }
                if tags.iter().any(|t| match *t {
                    Value::String(ref s) => !is_tag(s),
                    _ => unreachable!()})
                {
                    return Err(TagErrorKind::NotATag.into());
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
            _ => Err(TagErrorKind::TagTypeError.into()),
        }
    }

    fn set_tags(&mut self, ts: &[Tag]) -> Result<()> {
        if ts.iter().any(|tag| !is_tag(tag)) {
            debug!("Not a tag: '{}'", ts.iter().filter(|t| !is_tag(t)).next().unwrap());
            return Err(TagErrorKind::NotATag.into());
        }

        let a = ts.iter().unique().map(|t| Value::String(t.clone())).collect();
        self.set("imag.tags", Value::Array(a))
            .map(|_| ())
            .map_err(Box::new)
            .map_err(|e| TagErrorKind::HeaderWriteError.into_error_with_cause(e))
    }

    fn add_tag(&mut self, t: Tag) -> Result<()> {
        if !is_tag(&t) {
            debug!("Not a tag: '{}'", t);
            return Err(TagErrorKind::NotATag.into());
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
            return Err(TagErrorKind::NotATag.into());
        }

        self.get_tags()
            .map(|mut tags| {
                tags.retain(|tag| tag.clone() != t);
                self.set_tags(&tags[..])
            })
            .map(|_| ())
    }

    fn has_tag(&self, t: TagSlice) -> Result<bool> {
        let tags = try!(self.read("imag.tags").map_err_into(TagErrorKind::HeaderReadError));

        if !tags.iter().all(|t| is_match!(*t, Value::String(_))) {
            return Err(TagErrorKind::TagTypeError.into());
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
            result = result && try!(self.has_tag(tag));
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

