pub use std::path::Path;
pub use std::fs::File;

pub use runtime::Runtime;
pub use error::ImagError;
pub use error::ImagErrorBase;

pub struct StorageError {
    base : ImagErrorBase,
    backendname : String,
}

impl StorageError {

    pub fn new<T : StorageBackend>(backend : &T, short: String, long: String) -> StorageError {
        StorageError {
            base: ImagErrorBase {
                shortdesc: short,
                longdesc: long,
            },
            backendname: backend.name()
        }
    }

}

impl<'a> ImagError<'a> for StorageError {

    fn print(&self, rt: &Runtime) {
        if self.base.longdesc.is_empty() {
            let s = format!("Backend {}: {}\n\n{}\n\n",
                            self.backendname,
                            self.base.shortdesc,
                            self.base.longdesc);
            rt.print(&s)
        } else {
            let s = format!("Backend {}: {}\n",
                            self.backendname,
                            self.base.shortdesc);
            rt.print(&s)
        }
    }

    fn print_short(&self, rt : &Runtime) {
        let s = format!("Backend {}: {}\n",
                        self.backendname,
                        self.base.shortdesc);
        rt.print(&s)
    }

    fn print_long(&self, rt : &Runtime) {
        let s = format!("Backend {}: {}\n\n{}\n\n",
                        self.backendname,
                        self.base.shortdesc,
                        self.base.longdesc);
        rt.print(&s)
    }
}

pub trait StorageBackend {

    fn name(&self) -> String;

    fn create(&self, file : File)   -> Option<StorageError>;
    fn read(&self, path: Path)      -> Result<File, StorageError>;
    fn update(&self, file : File)   -> Option<StorageError>;
    fn destroy(&self, path: Path)   -> Option<StorageError>;

}
