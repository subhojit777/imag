use std::cell::RefCell;
use std::collections::HashMap;

pub mod path;
pub mod file;
pub mod parser;
pub mod json;

use storage::file::File;
use storage::file::id::FileID;

type Cache<'a> = HashMap<FileID, RefCell<File<'a>>>;

pub struct Store<'a> {
    cache : RefCell<Cache<'a>>,
}

impl<'a> Store<'a> {

    pub fn new() -> Store<'a> {
        Store {
            cache: RefCell::new(HashMap::new()),
        }
    }

}
