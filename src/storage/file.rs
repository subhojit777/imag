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

impl Display for FileHeaderSpec {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            &FileHeaderSpec::Null       => write!(fmt, "NULL"),
            &FileHeaderSpec::Bool       => write!(fmt, "Bool"),
            &FileHeaderSpec::Integer    => write!(fmt, "Integer"),
            &FileHeaderSpec::UInteger   => write!(fmt, "UInteger"),
            &FileHeaderSpec::Float      => write!(fmt, "Float"),
            &FileHeaderSpec::Text       => write!(fmt, "Text"),
            &FileHeaderSpec::Key{name: ref n, value_type: ref vt} => {
                write!(fmt, "Key({:?}) -> {:?}", n, vt)
            }
            &FileHeaderSpec::Array{allowed_types: ref at}  => {
                write!(fmt, "Array({:?})", at)
            }
        }
    }

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

