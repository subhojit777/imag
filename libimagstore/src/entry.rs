use header::EntryHeader;
use content::EntryContent;
use store::StoreId;

/**
 * An Entry of the store
 *
 * Contains location, header and content part.
 */
#[derive(Debug, Clone)]
pub struct Entry {
    location: StoreId,
    header: EntryHeader,
    content: EntryContent,
}

impl Entry {

    pub fn get_location(&self) -> &StoreId {
        &self.location
    }

    pub fn get_header(&self) -> &EntryHeader {
        &self.header
    }

    pub fn get_content(&self) -> &EntryContent {
        &self.content
    }

}

