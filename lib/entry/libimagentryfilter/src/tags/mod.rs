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

use libimagstore::store::Entry;
use libimagentrytag::tagable::Tagable;
use libimagentrytag::tag::Tag;

use filters::filter::Filter;

/// Check whether an Entry has a certain tag
pub struct HasTag {
    tag: Tag,
}

impl HasTag {

    pub fn new(tag: Tag) -> HasTag {
        HasTag {
            tag: tag,
        }
    }

}

impl Filter<Entry> for HasTag {

    fn filter(&self, e: &Entry) -> bool {
        e.has_tag(&self.tag).ok().unwrap_or(false)
    }

}


/// Check whether an Entry has all of these tags
pub struct HasAllTags {
    tags: Vec<Tag>,
}

impl HasAllTags {

    pub fn new(tags: Vec<Tag>) -> HasAllTags {
        HasAllTags {
            tags: tags,
        }
    }

}

impl Filter<Entry> for HasAllTags {

    fn filter(&self, e: &Entry) -> bool {
        e.has_tags(&self.tags).ok().unwrap_or(false)
    }

}


/// Check whether an Entry has any of these tags
pub struct HasAnyTags {
    tags: Vec<Tag>,
}

impl HasAnyTags {

    pub fn new(tags: Vec<Tag>) -> HasAnyTags {
        HasAnyTags {
            tags: tags,
        }
    }

}

impl Filter<Entry> for HasAnyTags {

    fn filter(&self, e: &Entry) -> bool {
        self.tags.iter().any(|tag| e.has_tag(tag).ok().unwrap_or(false))
    }

}

