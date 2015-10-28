use runtime::Runtime;
use std::error::Error;
use std::fmt::Formatter;
use std::fmt::Result as FMTResult;
use std::fmt::Display;
use std::path::Path;
use std::result::Result;

use module::todo::TodoModule;

mod todo;

#[derive(Debug)]
pub struct ModuleError {
    desc: String,
}

impl ModuleError {
    fn mk(desc: &'static str) -> ModuleError {
        ModuleError {
            desc: desc.to_owned().to_string(),
        }
    }
}

impl Error for ModuleError {

    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl Display for ModuleError {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        write!(f, "ModuleError: {}", self.description())
    }
}

pub type ModuleResult = Result<(), ModuleError>;

pub trait Module {

    fn new(rt : &Runtime) -> Self;
    fn callnames() -> &'static [&'static str];
    fn name(&self) -> &'static str;

    fn execute(&self, rt : &Runtime) -> ModuleResult;
    fn shutdown(&self, rt : &Runtime) -> ModuleResult;

}


pub mod file {

    use regex::Regex;

    pub struct ParserError {
        summary: String,
        parsertext: String,
        index: i32,
        explanation: Option<String>,
    }

    impl ParserError {
        fn new(sum: &'static str, text: String, idx: i32, expl: &'static str) -> ParserError {
            ParserError {
                summary: String::from(sum),
                parsertext: text,
                index: idx,
                explanation: Some(String::from(expl)),
            }
        }

        fn short(sum: &'static str, text: String, idx: i32) -> ParserError {
            ParserError {
                summary: String::from(sum),
                parsertext: text,
                index: idx,
                explanation: None
            }
        }
    }

    pub mod header {

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

        pub trait FileHeaderParser {
            fn new(spec: &FileHeaderSpec) -> Self;
            fn read(&self, string: &String) -> Result<FileHeaderData, super::ParserError>;
            fn write(&self, data: &FileHeaderData) -> Result<String, super::ParserError>;
        }

    }

    pub trait FileData {
        fn get_fulltext(&self) -> String;
        fn get_abbrev(&self) -> String;
    }

    pub trait FileParser {
        fn new(header_parser: &header::FileHeaderParser) -> FileParser;
        fn read(&self, string: String) -> (header::FileHeaderData, FileData);
        fn write(&self, hdr: &header::FileHeaderData, data: &FileData) -> Result<String, ParserError>;
    }

    pub type HeaderDataTpl = (Option<String>, Option<String>);

    pub fn divide_text(text: String) -> Result<HeaderDataTpl, ParserError> {
        let re = Regex::new(r"(?m)^\-\-\-$\n(.*)^\-\-\-$\n(.*)").unwrap();

        let captures = re.captures(&text[..]).unwrap_or(
            return Err(ParserError::new("Cannot run regex on text",
                                        text.clone(), 0,
                                        "Cannot run regex on text to divide it into header and content."))
        );

        if captures.len() != 2 {
            return Err(ParserError::new("Unexpected Regex output",
                                        text.clone(), 0,
                                        "The regex to divide text into header and content had an unexpected output."))
        }

        let header  = captures.at(0).map(|s| String::from(s));
        let content = captures.at(1).map(|s| String::from(s));
        Ok((header, content))
    }

}
