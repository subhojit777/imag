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
        // TODO: Don't use "/store" but value from configuration
        debug!("Building StorageBackend for {}", path);
        StorageBackend::new(path)
    }

    fn get_file_ids(&self) -> Option<Vec<FileID>> {
        debug!("Getting files from {}", self.basepath);
        let list = glob(&self.basepath[..]);

        if let Ok(globlist) = list {
            let mut v = vec![];
            for entry in globlist {
                if let Ok(path) = entry {
                    debug!(" - File: {:?}", path);
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
            debug!("    contents: {}", string);
            Ok(Ok(()))
        } else {
            debug!("Error parsing : {:?}", f.contents());
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
            debug!("Error parsing contents: {:?}", f.contents());
            return Err(contents.err().unwrap());
        }

        let content = contents.unwrap();
        debug!("Success parsing content : {}", content);

        let path = self.build_filepath(&f);
        debug!("Trying to write to file at {}", path);
        if let Err(_) = FSFile::open(&path) {
            debug!("Error opening {}", path);
            return Ok(Err(StorageBackendError::new(
                String::from("File::open()"),
                format!("Tried to open '{}'", path),
                String::from("Tried to update contents of this file, though file doesn't exist"),
                None)))
        }

        if let Ok(mut file) = FSFile::create(&path) {
            if let Err(writeerr) = file.write_all(&content.clone().into_bytes()) {
                debug!("Error writing to {}", path);
                return Ok(Err(StorageBackendError::new(
                    String::from("File::write()"),
                    format!("Tried to write '{}'", path),
                    String::from("Tried to write contents of this file, though operation did not succeed"),
                    Some(content))))
            }
        }

        debug!("Successfully written to file.");
        Ok(Ok(()))
    }

    /*
     * Find a file by its ID and return it if found. Return nothing if not
     * found, of course.
     *
     * TODO: Needs refactoring, as there might be an error when reading from
     * disk OR the id just does not exist.
     */
    pub fn get_file_by_id<'a, HP>(&self, id: FileID, p: &Parser<HP>) -> Option<File>
        where HP: FileHeaderParser<'a>
    {
        debug!("Searching for file with id '{}'", id);
        if let Ok(mut fs) = FSFile::open(self.build_filepath_with_id(id.clone())) {
            let mut s = String::new();
            fs.read_to_string(&mut s);
            debug!("Success reading file with id '{}'", id);
            debug!("Parsing to internal structure now");
            p.read(s).and_then(|(h, d)| Ok(File::from_parser_result(id, h, d))).ok()
        } else {
            debug!("No file with id '{}'", id);
            None
        }
    }

    fn build_filepath(&self, f: &File) -> String {
        self.build_filepath_with_id(f.id())
    }

    fn build_filepath_with_id(&self, id: FileID) -> String {
        debug!("Building filepath for id '{}'", id);
        self.basepath.clone() + &id[..]
    }

}

#[derive(Debug)]
pub struct StorageBackendError {
    pub action: String,             // The file system action in words
    pub desc: String,               // A short description
    pub explanation: String,        // A long, user friendly description
    pub data_dump: Option<String>   // Data dump, if any
}

impl StorageBackendError {
    fn new(action: String,
           desc  : String,
           explan: String,
           data  : Option<String>) -> StorageBackendError
    {
        StorageBackendError {
            action:         action,
            desc:           desc,
            explanation:    explan,
            data_dump:      data,
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

