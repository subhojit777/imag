use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FMTResult;
use std::fs::File as FSFile;
use std::fs::{create_dir_all, remove_file};
use std::io::{Read, Write};
use std::vec::{Vec, IntoIter};

use glob::glob;
use glob::Paths;

use module::Module;
use runtime::Runtime;
use storage::file::File;
use storage::file_id::*;
use storage::parser::{FileHeaderParser, Parser};

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
            Err(serr_build("create_dir_all()", "Could not create store directories",
                           Some(storepath), Some(Box::new(e))))
        })
    }

    pub fn iter_ids(&self, m: &Module) -> Result<IntoIter<FileID>, StorageBackendError>
    {
        let globstr = self.prefix_of_files_for_module(m) + "*.imag";
        debug!("Globstring = {}", globstr);
        glob(&globstr[..])
            .and_then(|globlist| {
                debug!("Iterating over globlist");
                Ok(globlist_to_file_id_vec(globlist).into_iter())
            })
            .map_err(|e| {
                debug!("glob() returned error: {:?}", e);
                serr_build("iter_ids()", "Cannot iter on file ids",
                        None, None)
            })
    }

    pub fn iter_files<'a, HP>(&self, m: &'a Module, p: &Parser<HP>)
        -> Result<IntoIter<File<'a>>, StorageBackendError>
        where HP: FileHeaderParser
    {
        self.iter_ids(m)
            .and_then(|ids| {
                debug!("Iterating ids and building files from them");
                debug!("  number of ids = {}", ids.len());
                Ok(self.filter_map_ids_to_files(m, p, ids).into_iter())
            })
            .map_err(|e| {
                debug!("StorageBackend::iter_ids() returned error = {:?}", e);
                serr_build("iter_files()", "Cannot iter on files",
                            None, Some(Box::new(e)))
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
                    serr_build("File::write_all()",
                               "Could not write out File contents",
                               None, Some(Box::new(ioerr)))
                })
        }).map_err(|writeerr| {
            debug!("Could not create file at '{}'", path);
            serr_build("File::create()", "Creating file on disk failed",
                       None, Some(Box::new(writeerr)))
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
                    serr_build("File::write()",
                               "Tried to write contents of this file, though operation did not succeed",
                                Some(string), Some(Box::new(ioerr)))
                })
        }).map_err(|writeerr| {
            debug!("Could not write file at '{}'", path);
            serr_build("File::open()",
                       "Tried to update contents of this file, though file doesn't exist",
                       None, Some(Box::new(writeerr)))
        }).and(Ok(()))
    }

    /*
     * Find a file by its ID and return it if found. Return nothing if not
     * found, of course.
     *
     * TODO: Needs refactoring, as there might be an error when reading from
     * disk OR the id just does not exist.
     */
    pub fn get_file_by_id<'a, HP>(&self, m: &'a Module, id: &FileID, p: &Parser<HP>) -> Option<File<'a>>
        where HP: FileHeaderParser
    {
        debug!("Searching for file with id '{}'", id);

        if id.get_type() == FileIDType::NONE {
            // We don't know the hash type, so we glob() around a bit.
            debug!("Having FileIDType::NONE, so we glob() for the raw ID");

            let id_str = id.get_id().unwrap_or(String::from("INVALID"));
            let globstr = self.prefix_of_files_for_module(m) + "*" + &id_str[..] + ".imag";
            debug!("Globbing with globstr = '{}'", globstr);
            glob(&globstr[..]).map(|globlist| {
                let idvec = globlist_to_file_id_vec(globlist).into_iter();
                let mut vec = self.filter_map_ids_to_files(m, p, idvec);
                vec.reverse();
                vec.pop()
            }).unwrap_or({
                debug!("No glob matches, actually. We can't do anything at this point");
                None
            })
        } else {
            // The (hash)type is already in the FileID object, so we can just
            // build a path from the information we already have
            debug!("We know FileIDType, so we build the path directly now");
            let filepath = self.build_filepath_with_id(m, id.clone());
            if let Ok(mut fs) = FSFile::open(filepath) {
                let mut s = String::new();
                fs.read_to_string(&mut s);

                debug!("Success opening file with id '{}'", id);
                debug!("Parsing to internal structure now");
                p.read(s).and_then(|(h, d)| {
                    Ok(File::from_parser_result(m, id.clone(), h, d))
                }).ok()
            } else {
                debug!("No file with id '{}'", id);
                None
            }
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
            serr_build("remove_file()", "File removal failed",
                       Some(format!("{}", file)), Some(Box::new(e)))
        })
    }

    fn build_filepath(&self, f: &File) -> String {
        self.build_filepath_with_id(f.owner(), f.id())
    }

    fn build_filepath_with_id(&self, owner: &Module, id: FileID) -> String {
        let idstr   : String        = id.clone().into();
        let idtype  : FileIDType    = id.into();
        let typestr : String        = idtype.into();

        debug!("Building filepath with id");
        debug!("  basepath: '{}'", self.basepath);
        debug!(" storepath: '{}'", self.storepath);
        debug!("        id: '{}'", idstr);
        debug!("      type: '{}'", typestr);

        self.prefix_of_files_for_module(owner) +
            "-" + &typestr[..] +
            "-" + &idstr[..] +
            ".imag"
    }

    fn prefix_of_files_for_module(&self, m: &Module) -> String {
        self.storepath.clone() + m.name()
    }

    fn filter_map_ids_to_files<'a, HP>(&self,
                                        m: &'a Module,
                                        p: &Parser<HP>,
                                        ids: IntoIter<FileID>)
        -> Vec<File<'a>>
        where HP: FileHeaderParser
    {
        ids.filter_map(|id| self.get_file_by_id(m, &id, p))
           .collect::<Vec<File>>()
    }

}

#[derive(Debug)]
pub struct StorageBackendError {
    pub action: String,             // The file system action in words
    pub desc: String,               // A short description
    pub data_dump: Option<String>,  // Data dump, if any
    pub caused_by: Option<Box<Error>>,  // caused from this error
}

impl StorageBackendError {

    fn new<S>(action: S, desc: S, data: Option<String>) -> StorageBackendError
        where S: Into<String>
    {
        StorageBackendError {
            action:         action.into(),
            desc:           desc.into(),
            data_dump:      data,
            caused_by:      None,
        }
    }

}

impl Error for StorageBackendError {

    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&Error> {
        self.caused_by.as_ref().map(|e| &**e)
    }

}

impl<'a> Display for StorageBackendError {
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
            Err(serr_build("Parser::write()",
                           "Cannot translate internal representation of file contents into on-disk representation",
                           None, Some(Box::new(err))))
        })
}

fn globlist_to_file_id_vec(globlist: Paths) -> Vec<FileID> {
    globlist.filter_map(Result::ok)
            .map(|pbuf| FileID::from(&pbuf))
            .collect::<Vec<FileID>>()
}

/*
 * Helper to build a StorageBackendError object with cause, because one line is
 * less than three lines
 */
fn serr_build(action: &'static str, desc: &'static str,
              data: Option<String>, caused_by: Option<Box<Error>>)
    -> StorageBackendError
{
    let mut err = StorageBackendError::new(action, desc, data);
    err.caused_by = caused_by;
    err
}

