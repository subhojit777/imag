use std::path::PathBuf;
use std::result::Result as RResult;

pub use entry::Entry;
pub use error::StoreError;

pub type Result<T> = RResult<T, StoreError>;

pub struct Store {
    location: PathBuf,
}

impl Store {

    pub fn create(entry: Entry) -> Result<()> {
        unimplemented!()
    }

    pub fn read(path: PathBuf) -> Result<Entry> {
        unimplemented!()
    }

    pub fn update(entry: Entry) -> Result<()> {
        unimplemented!()
    }

    pub fn delete(path: PathBuf) -> Result<()> {
        unimplemented!()
    }

}

