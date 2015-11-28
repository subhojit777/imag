use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FMTResult;
use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;
use std::fs::File as FSFile;
use std::fs::create_dir_all;
use std::fs::remove_file;
use std::io::Read;
use std::io::Write;
use std::vec::IntoIter;

use glob::glob;
use glob::Paths;

use storage::file::File;
use storage::file_id::*;
use storage::parser::{FileHeaderParser, Parser, ParserError};

use module::Module;
use runtime::Runtime;

pub type BackendOperationResult<T = ()> = Result<T, StorageBackendError>;

pub struct StorageBackend {
    basepath: String,
    storepath: String,
}

impl StorageBackend {

    pub fn new(rt: &Runtime) -> BackendOperationResult<StorageBackend> {
        let storepath = rt.get_rtp() + "/store/";
        debug!("Trying to create {}", storepath);
        create_dir_all(&storepath).and_then(|_| {
            debug!("Creating succeeded, constructing backend instance");
            Ok(StorageBackend {
                basepath: rt.get_rtp(),
                storepath: storepath.clone(),
            })
        }).or_else(|e| {
            debug!("Creating failed, constructing error instance");
            let mut serr = StorageBackendError::build(
                "create_dir_all()",
                "Could not create store directories",
                Some(storepath)
            );
            serr.caused_by = Some(Box::new(e));
            Err(serr)
        })
    }

    fn build<M: Module>(rt: &Runtime, m: &M) -> StorageBackend {
        let path = rt.get_rtp() + m.name() + "/store";
        // TODO: Don't use "/store" but value from configuration
        debug!("Building StorageBackend for {}", path);
        StorageBackend::new(path)
    }

    fn get_file_ids(&self, m: &Module) -> Option<Vec<FileID>> {
        let list = glob(&self.prefix_of_files_for_module(m)[..]);

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

    pub fn iter_ids(&self, m: &Module) -> Option<IntoIter<FileID>>
    {
        glob(&self.prefix_of_files_for_module(m)[..]).and_then(|globlist| {
            let v = globlist.filter_map(Result::ok)
                            .map(|pbuf| from_pathbuf(&pbuf))
                            .collect::<Vec<FileID>>()
                            .into_iter();
            Ok(v)
        }).ok()
    }

    pub fn iter_files<'a, HP>(&self, m: &'a Module, p: &Parser<HP>)
        -> Option<IntoIter<File<'a>>>
        where HP: FileHeaderParser
    {
        self.iter_ids(m).and_then(|ids| {
            Some(ids.filter_map(|id| self.get_file_by_id(m, &id, p))
                    .collect::<Vec<File>>()
                    .into_iter())
        })
    }

    /*
     * Write a file to disk.
     *
     * The file is moved to this function as the file won't be edited afterwards
     */
    pub fn put_file<HP>(&self, f: File, p: &Parser<HP>) -> BackendOperationResult
        where HP: FileHeaderParser
    {
        let written = write_with_parser(&f, p);
        if written.is_err() { return Err(written.err().unwrap()); }
        let string = written.unwrap();

        let path = self.build_filepath(&f);
        debug!("Writing file: {}", path);
        debug!("      string: {}", string);

        FSFile::create(&path).map(|mut file| {
            debug!("Created file at '{}'", path);
            file.write_all(&string.clone().into_bytes())
                .map_err(|ioerr| {
                    debug!("Could not write file");
                    let mut err = StorageBackendError::build(
                            "File::write_all()",
                            "Could not write out File contents",
                            None
                        );
                    err.caused_by = Some(Box::new(ioerr));
                    err
                })
        }).map_err(|writeerr| {
            debug!("Could not create file at '{}'", path);
            let mut err = StorageBackendError::build(
                "File::create()",
                "Creating file on disk failed",
                None
            );
            err.caused_by = Some(Box::new(writeerr));
            err
        }).and(Ok(()))
    }

    /*
     * Update a file. We have the UUID and can find the file on FS with it and
     * then replace its contents with the contents of the passed file object
     */
    pub fn update_file<HP>(&self, f: File, p: &Parser<HP>) -> BackendOperationResult
        where HP: FileHeaderParser
    {
        let contents = write_with_parser(&f, p);
        if contents.is_err() { return Err(contents.err().unwrap()); }
        let string = contents.unwrap();

        let path = self.build_filepath(&f);
        debug!("Writing file: {}", path);
        debug!("      string: {}", string);

        FSFile::open(&path).map(|mut file| {
            debug!("Open file at '{}'", path);
            file.write_all(&string.clone().into_bytes())
                .map_err(|ioerr| {
                    debug!("Could not write file");
                    let mut err = StorageBackendError::build(
                            "File::write()",
                            "Tried to write contents of this file, though operation did not succeed",
                            Some(string)
                        );
                    err.caused_by = Some(Box::new(ioerr));
                    err
                })
        }).map_err(|writeerr| {
            debug!("Could not write file at '{}'", path);
            let mut err = StorageBackendError::build(
                "File::open()",
                "Tried to update contents of this file, though file doesn't exist",
                None
            );
            err.caused_by = Some(Box::new(writeerr));
            err
        }).and(Ok(()))
    }

    /*
     * Find a file by its ID and return it if found. Return nothing if not
     * found, of course.
     *
     * TODO: Needs refactoring, as there might be an error when reading from
     * disk OR the id just does not exist.
     */
    pub fn get_file_by_id<'a, HP>(&self, m: &'a Module, id: FileID, p: &Parser<HP>) -> Option<File<'a>>
        where HP: FileHeaderParser
    {
        debug!("Searching for file with id '{}'", id);
        if let Ok(mut fs) = FSFile::open(self.build_filepath_with_id(m, id.clone())) {
            let mut s = String::new();
            fs.read_to_string(&mut s);
            debug!("Success reading file with id '{}'", id);
            debug!("Parsing to internal structure now");
            p.read(s).and_then(|(h, d)| Ok(File::from_parser_result(m, id.clone(), h, d))).ok()
        } else {
            debug!("No file with id '{}'", id);
            None
        }
    }

    pub fn remove_file(&self, m: &Module, file: File, checked: bool) -> BackendOperationResult {
        if checked {
            error!("Checked remove not implemented yet. I will crash now");
            unimplemented!()
        }

        debug!("Doing unchecked remove");
        info!("Going to remove file: {}", file);

        let fp = self.build_filepath(&file);
        remove_file(fp).map_err(|e| {
            let mut serr = StorageBackendError::build(
                "remove_file()",
                "File removal failed",
                Some(format!("{}", file))
            );
            serr.caused_by = Some(Box::new(e));
            serr
        })
    }

    fn build_filepath(&self, f: &File) -> String {
        self.build_filepath_with_id(f.owner(), f.id())
    }

    fn build_filepath_with_id(&self, owner: &Module, id: FileID) -> String {
        debug!("Building filepath with id");
        debug!("  basepath: '{}'", self.basepath);
        debug!(" storepath: '{}'", self.storepath);
        debug!("  id      : '{}'", id);
        self.storepath.clone() + owner.name() + "-" + &id[..] + ".imag"
    }

}

#[derive(Debug)]
pub struct StorageBackendError<'a> {
    pub action: String,             // The file system action in words
    pub desc: String,               // A short description
    pub data_dump: Option<String>,  // Data dump, if any
    pub caused_by: Option<Box<Error>>,  // caused from this error
}

impl<'a> StorageBackendError<'a> {
    fn new(action: String,
           desc  : String,
           data  : Option<String>) -> StorageBackendError<'a>
    {
        StorageBackendError {
            action:         action,
            desc:           desc,
            data_dump:      data,
            caused_by:      None,
        }
    }

    fn build(action: &'static str,
             desc:   &'static str,
           data  : Option<String>) -> StorageBackendError
    {
        StorageBackendError {
            action:         String::from(action),
            desc:           String::from(desc),
            dataDump:       data,
            caused_by:      None,
        }
    }

}

impl<'a> Error for StorageBackendError<'a> {

    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&Error> {
        self.caused_by.as_ref().map(|e| &**e)
    }

}

impl<'a> Display for StorageBackendError<'a> {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        write!(f, "StorageBackendError[{}]: {}",
               self.action, self.desc)
    }
}


fn write_with_parser<'a, HP>(f: &File, p: &Parser<HP>) -> Result<String, StorageBackendError>
    where HP: FileHeaderParser
{
    p.write(f.contents())
        .or_else(|err| {
            let mut serr = StorageBackendError::build(
                "Parser::write()",
                "Cannot translate internal representation of file contents into on-disk representation",
                None
            );
            serr.caused_by = Some(Box::new(err));
            Err(serr)
        })
}
