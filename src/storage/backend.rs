use std::error::Error;
use std::fmt::Display;

use super::file::FileID;
use super::file::File;
use module::Module;

type BackendOperationResult = Result<(), StorageBackendError>;

pub struct StorageBackend<'a> {
    basepath: String,
    module: &'a Module,
}

impl<'a> StorageBackend<'a> {

    fn new() -> StorageBackend<'a> {
    }

    fn getFileList() -> Vec<(String, FileID)> {
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

#[derive(Debug)]
pub struct StorageBackendError;

impl StorageBackendError {
}

impl Error for StorageBackendError {
}

impl Display for StorageBackendError {
}

