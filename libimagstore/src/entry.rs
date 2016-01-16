use std::path::PathBuf;

use header::EntryHeader;
use content::EntryContent;

#[derive(Debug, Clone)]
pub struct Entry {
    location: PathBuf,
    header: EntryHeader,
    content: EntryContent,
}

