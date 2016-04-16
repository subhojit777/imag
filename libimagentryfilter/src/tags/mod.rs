use libimagstore::store::Entry;
use libimagentrytag::tagable::Tagable;
use libimagentrytag::tag::Tag;

use filter::Filter;

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

impl Filter for HasTag {

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

impl Filter for HasAllTags {

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

impl Filter for HasAnyTags {

    fn filter(&self, e: &Entry) -> bool {
        self.tags.iter().any(|tag| e.has_tag(tag).ok().unwrap_or(false))
    }

}

