use std::cell::RefCell;
use std::collections::HashMap;

pub mod path;
pub mod file;
pub mod parser;
pub mod json;

use module::Module;
use storage::file::File;
use storage::file::id::FileID;
use storage::file::id_type::FileIDType;
use storage::file::hash::FileHash;
use storage::parser::{FileHeaderParser, Parser, ParserError};
use storage::file::header::data::FileHeaderData;

type Cache<'a> = HashMap<FileID, RefCell<File<'a>>>;

pub struct Store<'a> {
    cache : RefCell<Cache<'a>>,
}

impl<'a> Store<'a> {

    pub fn new() -> Store<'a> {
        Store {
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_file(&self, module: &'a Module<'a>) -> File<'a> {
        let f = File {
            owning_module: module,
            header: FileHeaderData::Null,
            data: String::from(""),
            id: self.get_new_file_id(),
        };
        debug!("Create new File object: {:?}", f);
        f
    }

    pub fn new_file_from_parser_result(&self, module: &'a Module<'a>, id: FileID, header: FileHeaderData, data: String) -> File<'a> {
        let f = File {
            owning_module: module,
            header: header,
            data: data,
            id: id,
        };
        debug!("Create new File object from parser result: {:?}", f);
        f
    }

    pub fn new_file_with_header(&self, module: &'a Module<'a>, h: FileHeaderData) -> File<'a> {
        let f = File {
            owning_module: module,
            header: h,
            data: String::from(""),
            id: self.get_new_file_id(),
        };
        debug!("Create new File object with header: {:?}", f);
        f
    }

    pub fn new_file_with_data(&self, module: &'a Module<'a>, d: String) -> File<'a> {
        let f = File {
            owning_module: module,
            header: FileHeaderData::Null,
            data: d,
            id: self.get_new_file_id(),
        };
        debug!("Create new File object with data: {:?}", f);
        f
    }

    pub fn new_file_with_content(&self, module: &'a Module<'a>, h: FileHeaderData, d: String) -> File<'a> {
        let f = File {
            owning_module: module,
            header: h,
            data: d,
            id: self.get_new_file_id(),
        };
        debug!("Create new File object with content: {:?}", f);
        f
    }

    pub fn persist(&self, file: &File) -> bool {
        unimplemented!()
    }

    pub fn load(&self, id: FileID) -> File {
        unimplemented!()
    }

    fn get_new_file_id(&self) -> FileID {
        use uuid::Uuid;
        let hash = FileHash::from(Uuid::new_v4().to_hyphenated_string());
        FileID::new(FileIDType::UUID, hash)
    }

}
