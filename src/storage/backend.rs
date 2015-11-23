use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FMTResult;
use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;

use glob::glob;
use glob::Paths;

use storage::file::{File, FileID};
use module::Module;

type BackendOperationResult = Result<(), StorageBackendError>;

pub struct StorageBackend<'a> {
    basepath: String,
    module: &'a Module,
}

impl<'a> StorageBackend<'a> {

    fn new(basepath: String, module: &'a Module) -> StorageBackend<'a> {
        StorageBackend {
            basepath: basepath,
            module: module,
        }
    }

    fn getFileList(&self) -> Option<Vec<FileID>> {
        let list = glob(&self.basepath[..]);

        if let Ok(globlist) = list {
            let mut v = vec![];
            for entry in globlist {
                if let Ok(path) = entry {
                    v.push(file_id_from_path(path.as_path()));
                } else {
                    // Entry is not a path
                }
            }

            Some(v)
        } else {
            None
        }
    }

    fn createEmpty() -> FileID {
    }

    fn createFile() -> File {
    }

    fn writeFile(f: File) -> BackendOperationResult {
    }

    fn createFileWithContent(content: String) -> BackendOperationResult {
    }

    fn readFile(id: FileID) -> String {
    }

    // TODO: Meta files are not covered yet

}

fn file_id_from_path(p: &Path) -> String {
    String::from("")
}

#[derive(Debug)]
pub struct StorageBackendError {
    pub action: String,             // The file system action in words
    pub desc: String,               // A short description
    pub explanation: String,        // A long, user friendly description
    pub dataDump: Option<String>    // Data dump, if any
}

impl StorageBackendError {
    fn new(action: &'static str,
           desc  : &'static str,
           explan: &'static str,
           data  : Option<String>) -> StorageBackendError
    {
        StorageBackendError {
            action:         String::from(action),
            desc:           String::from(desc),
            explanation:    String::from(explan),
            dataDump:       data,
        }
    }
}

impl Error for StorageBackendError {

    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl Display for StorageBackendError {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        write!(f, "StorageBackendError[{}]: {}\n\n{}",
               self.action, self.desc, self.explanation)
    }
}

