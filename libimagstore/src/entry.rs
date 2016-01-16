use header::EntryHeader;
use content::EntryContent;
use store::StoreId;

#[derive(Debug, Clone)]
pub struct Entry {
    location: StoreId,
    header: EntryHeader,
    content: EntryContent,
}

