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
pub struct File {
    pub owning_module_name  : &'static str,
    pub header              : FileHeaderData,
    pub data                : String,
    pub id                  : FileID,
}

impl File {

    pub fn owner_name(&self) -> &'static str {
        self.owning_module_name
    }

    pub fn header(&self) -> &FileHeaderData {
        &self.header
    }

    pub fn set_header(&mut self, new_header: FileHeaderData) {
        self.header = new_header;
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn contents(&self) -> (&FileHeaderData, &String) {
        (self.header(), self.data())
    }

    pub fn id(&self) -> &FileID {
        &self.id
    }

    pub fn matches_with(&self, r: &Regex) -> bool {
        r.is_match(&self.data[..]) || self.header.matches_with(r)
    }

}

impl Display for File {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt,
"[File] Owner : '{:?}'
        FileID: '{:?}'
        Header: '{:?}'
        Data  : '{:?}'",
               self.owning_module_name,
               self.header,
               self.data,
               self.id);
        Ok(())
    }

}

impl Debug for File {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt,
"[File] Owner : '{:?}'
        FileID: '{:?}'
        Header: '{:?}'
        Data  : '{:?}'",
               self.owning_module_name,
               self.id,
               self.header,
               self.data);
        Ok(())
    }

}

#[cfg(test)]
mod test {
    // we use the JSON parser here, so we can generate FileHeaderData
    use storage::json::parser::JsonHeaderParser;
    use storage::file::header::match_header_spec;
    use storage::parser::{FileHeaderParser, ParserError};
    use storage::file::header::data::FileHeaderData as FHD;
    use storage::file::header::spec::FileHeaderSpec as FHS;

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

