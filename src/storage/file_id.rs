use std::path::{Path, PathBuf};

pub type FileID = String;

pub fn from_path_string(s: &String) -> FileID {
    String::from("")
}

pub fn from_path(p: &Path) -> FileID {
    String::from("")
}

pub fn from_pathbuf(p: &PathBuf) -> FileID {
    from_path(p.as_path())
}

