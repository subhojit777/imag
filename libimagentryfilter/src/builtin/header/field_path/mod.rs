use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;
use std::error::Error;

use toml::Value;

pub mod element;
pub mod error;

use libimagstore::store::Entry;
use libimagstore::store::EntryHeader;

use builtin::header::field_path::element::FieldPathElement;
use builtin::header::field_path::error::FieldPathParsingError;

pub struct FieldPath {
    elements: Vec<FieldPathElement>,
}

impl FieldPath {

    pub fn new(elements: Vec<FieldPathElement>) -> FieldPath {
        unimplemented!()
    }

    pub fn compile(source: String) -> Result<FieldPath, FieldPathParsingError> {
        unimplemented!()
    }

    pub fn walk(&self, e: &EntryHeader) -> Option<Value> {
        unimplemented!()
    }

}

