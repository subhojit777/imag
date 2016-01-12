use std::path::PathBuf;

use header::EntryHeader;
use content::EntryContent;

pub struct Entry {
    location: PathBuf,
    header: EntryHeader,
    content: EntryContent,
}

