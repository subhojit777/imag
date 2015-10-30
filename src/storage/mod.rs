pub use std::path::Path;
pub use std::fs::File;
pub use std::error::Error;

pub use runtime::Runtime;

mod file;

pub trait StorageBackend {

    fn name(&self) -> String;

    fn create(&self, file : File)   -> Option<Error>;
    fn read(&self, path: Path)      -> Result<File, Error>;
    fn update(&self, file : File)   -> Option<Error>;
    fn destroy(&self, path: Path)   -> Option<Error>;

}
