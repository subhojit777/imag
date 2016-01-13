use std::collections::HashMap;
use std::fs::File;
use std::ops::Drop;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::sync::Arc;
use std::sync::RwLock;

pub use entry::Entry;
pub use error::StoreError;

pub type Result<T> = RResult<T, StoreError>;

pub trait Store {

    fn location(&self) -> &PathBuf;

    fn create(&self, entry: Entry) -> Result<()>;
    fn read(&self, path: PathBuf) -> Result<Arc<RwLock<Entry>>>;
    fn update(&self, entry: Arc<RwLock<Entry>>) -> Result<()>;
    fn delete(&self, path: PathBuf) -> Result<()>;

}

