use std::result::Result as RResult;
use std::string;

pub use entry::Entry;
pub use error::StoreError;
pub use single_use_lock::SingleUseLock;

pub type Result<T> = RResult<T, StoreError>;
pub type LockedEntry = SingleUseLock<Entry>;

pub trait Store {
    fn create(&self, entry : Entry) -> Result<()>;
    fn retrieve(&self, id : string) -> Result<LockedEntry>;
    fn retrieve_copy(&self, id : string) -> Result<Entry>;
    fn update(&self, LockedEntry) -> Result<()>;
    fn delete(&self, id : string) -> Result<()>;
}

