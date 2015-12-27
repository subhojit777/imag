use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use regex::Regex;

pub mod id;
pub mod id_type;
pub mod header;
pub mod hash;


use module::Module;
use storage::file::id::*;
use storage::file::id_type::FileIDType;
use storage::file::hash::FileHash;
use super::parser::{FileHeaderParser, Parser, ParserError};

use self::header::spec::*;
use self::header::data::*;

/*
 * Internal abstract view on a file. Does not exist on the FS and is just kept
 * internally until it is written to disk.
 */
pub struct File<'a> {
    owning_module   : &'a Module<'a>,
    header          : FileHeaderData,
    data            : String,
    id              : FileID,
}

impl<'a> File<'a> {

    pub fn new(module: &'a Module<'a>) -> File<'a> {
        let f = File {
            owning_module: module,
            header: FileHeaderData::Null,
            data: String::from(""),
            id: File::get_new_file_id(),
        };
        debug!("Create new File object: {:?}", f);
        f
    }

    pub fn from_parser_result(module: &'a Module<'a>, id: FileID, header: FileHeaderData, data: String) -> File<'a> {
        let f = File {
            owning_module: module,
            header: header,
            data: data,
            id: id,
        };
        debug!("Create new File object from parser result: {:?}", f);
        f
    }

    pub fn new_with_header(module: &'a Module<'a>, h: FileHeaderData) -> File<'a> {
        let f = File {
            owning_module: module,
            header: h,
            data: String::from(""),
            id: File::get_new_file_id(),
        };
        debug!("Create new File object with header: {:?}", f);
        f
    }

    pub fn new_with_data(module: &'a Module<'a>, d: String) -> File<'a> {
        let f = File {
            owning_module: module,
            header: FileHeaderData::Null,
            data: d,
            id: File::get_new_file_id(),
        };
        debug!("Create new File object with data: {:?}", f);
        f
    }

    pub fn new_with_content(module: &'a Module<'a>, h: FileHeaderData, d: String) -> File<'a> {
        let f = File {
            owning_module: module,
            header: h,
            data: d,
            id: File::get_new_file_id(),
        };
        debug!("Create new File object with content: {:?}", f);
        f
    }

    pub fn header(&self) -> FileHeaderData {
        self.header.clone()
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub fn contents(&self) -> (FileHeaderData, String) {
        (self.header(), self.data())
    }

    pub fn id(&self) -> FileID {
        self.id.clone()
    }

    pub fn owner(&self) -> &'a Module<'a> {
        self.owning_module
    }

    pub fn matches_with(&self, r: &Regex) -> bool {
        r.is_match(&self.data[..]) || self.header.matches_with(r)
    }

    fn get_new_file_id() -> FileID {
        use uuid::Uuid;
        let hash = FileHash::from(Uuid::new_v4().to_hyphenated_string());
        FileID::new(FileIDType::UUID, hash)
    }
}

impl<'a> Display for File<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt,
"[File] Owner : '{:?}'
        FileID: '{:?}'
        Header: '{:?}'
        Data  : '{:?}'",
               self.owning_module,
               self.header,
               self.data,
               self.id);
        Ok(())
    }

}

impl<'a> Debug for File<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt,
"[File] Owner : '{:?}'
        FileID: '{:?}'
        Header: '{:?}'
        Data  : '{:?}'",
               self.owning_module,
               self.header,
               self.data,
               self.id);
        Ok(())
    }

}

#[cfg(test)]
mod test {
    // we use the JSON parser here, so we can generate FileHeaderData
    use storage::json::parser::JsonHeaderParser;
    use super::match_header_spec;
    use storage::parser::{FileHeaderParser, ParserError};
    use storage::file::FileHeaderData as FHD;
    use storage::file::FileHeaderSpec as FHS;

    #[test]
    fn test_spec_matching() {
        let text = String::from("{\"a\": 1, \"b\": -2}");
        let spec = FHS::Map {
            keys: vec![
                FHS::Key {
                    name: String::from("a"),
                    value_type: Box::new(FHS::UInteger)
                },
                FHS::Key {
                    name: String::from("b"),
                    value_type: Box::new(FHS::Integer)
                }
            ]
        };

        let parser  = JsonHeaderParser::new(Some(spec.clone()));
        let datares = parser.read(Some(text.clone()));
        assert!(datares.is_ok(), "Text could not be parsed: '{}'", text);
        let data = datares.unwrap();

        let matchres = match_header_spec(&spec, &data);
        assert!(matchres.is_none(), "Matching returns error: {:?}", matchres);
    }
}

