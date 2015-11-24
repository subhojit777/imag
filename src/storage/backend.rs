use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FMTResult;
use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;
use std::fs::File as FSFile;
use std::io::Read;
use std::io::Write;

use glob::glob;
use glob::Paths;

use storage::file::File;
use storage::file_id::*;
use storage::parser::{FileHeaderParser, Parser, ParserError};

use module::Module;
use runtime::Runtime;

pub type BackendOperationResult = Result<(), StorageBackendError>;

pub struct StorageBackend {
    basepath: String,
}

impl StorageBackend {

    pub fn new(basepath: String) -> StorageBackend {
        StorageBackend {
            basepath: basepath,
        }
    }

    fn build<M: Module>(rt: &Runtime, m: &M) -> StorageBackend {
        let path = rt.get_rtp() + m.name() + "/store";
        StorageBackend::new(path)
    }

    fn get_file_ids(&self) -> Option<Vec<FileID>> {
        let list = glob(&self.basepath[..]);

        if let Ok(globlist) = list {
            let mut v = vec![];
            for entry in globlist {
                if let Ok(path) = entry {
                    v.push(from_pathbuf(&path));
                } else {
                    // Entry is not a path
                }
            }

            Some(v)
        } else {
            None
        }
    }

    /*
     * Write a file to disk.
     *
     * The file is moved to this function as the file won't be edited afterwards
     */
    pub fn put_file<'a, HP>(&self, f: File, p: &Parser<HP>) ->
        Result<BackendOperationResult, ParserError>
        where HP: FileHeaderParser<'a>
    {
        let written = p.write(f.contents());
        if let Ok(string) = written {
            let path = self.build_filepath(&f);
            debug!("Writing file: {}", path);
            Ok(Ok(()))
        } else {
            Err(written.err().unwrap())
        }
    }

    /*
     * Update a file. We have the UUID and can find the file on FS with it and
     * then replace its contents with the contents of the passed file object
     */
    pub fn update_file<'a, HP>(&self, f: File, p: &Parser<HP>)
        -> Result<BackendOperationResult, ParserError>
        where HP: FileHeaderParser<'a>
    {
        let contents = p.write(f.contents());

        if contents.is_err() {
            return Err(contents.err().unwrap());
        }

        let content = contents.unwrap();

        let path = self.build_filepath(&f);
        if let Err(_) = FSFile::open(path) {
            return Ok(Err(StorageBackendError::new(
                "File::open()",
                &format!("Tried to open '{}'", path)[..],
                "Tried to update contents of this file, though file doesn't exist",
                None)))
        }

        if let Ok(mut file) = FSFile::create(path) {
            if let Err(writeerr) = file.write_all(&content.into_bytes()) {
                return Ok(Err(StorageBackendError::new(
                    "File::write()",
                    &format!("Tried to write '{}'", path)[..],
                    "Tried to write contents of this file, though operation did not succeed",
                    Some(content))))
            }
        }

        Ok(Ok(()))
    }

    /*
     * Find a file by its ID and return it if found. Return nothing if not
     * found, of course.
     *
     * TODO: Needs refactoring, as there might be an error when reading from
     * disk OR the id just does not exist.
     */
    pub fn get_file_by_id(id: FileID) -> Option<File> {
    }

    fn build_filepath(&self, f: &File) -> String {
        self.basepath + &f.id()[..]
    }

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

