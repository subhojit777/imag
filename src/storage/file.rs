use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub enum FileHeaderSpec {
    Null,
    Bool,
    Integer,
    UInteger,
    Float,
    Text,
    Key { name: String, value_type: Box<FileHeaderSpec> },
    Array { allowed_types: Box<Vec<FileHeaderSpec>> },
}

#[derive(Debug)]
pub enum FileHeaderData {
    Null,
    Bool(bool),
    Integer(i64),
    UInteger(u64),
    Float(f64),
    Text(String),
    Key { name: String, value: Box<FileHeaderData> },
    Array { values: Box<Vec<FileHeaderData>> },
}

pub trait FileData : Sized {
    fn get_fulltext(&self) -> String;
    fn get_abbrev(&self) -> String;
}

pub struct MatchError {
    summary: String,
    path: Vec<FileHeaderSpec>,
    expected: FileHeaderSpec,
    found: FileHeaderSpec
}

impl MatchError {
    pub fn format(&self) -> String {
        format!("MatchError: {:?}\n\nHaving: {:?}\nExpected: {:?}\nFound: {:?}\n",
               self.summary, self.path, self.expected, self.found)
    }
}

impl Error for MatchError {

    fn description(&self) -> &str {
        &self.summary[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl Debug for MatchError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.format());
        Ok(())
    }

}

impl Display for MatchError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.format());
        Ok(())
    }

}

