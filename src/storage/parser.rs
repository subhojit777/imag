use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use regex::Regex;

use super::file::header::data::FileHeaderData;

pub struct ParserError {
    summary: String,
    parsertext: String,
    index: i32,
    explanation: Option<String>,
    caused_by: Option<Box<Error>>,
}

impl ParserError {
    pub fn new(sum: &'static str, text: String, idx: i32, expl: &'static str) -> ParserError {
        ParserError {
            summary: String::from(sum),
            parsertext: text,
            index: idx,
            explanation: Some(String::from(expl)),
            caused_by: None,
        }
    }

    pub fn short(sum: &str, text: String, idx: i32) -> ParserError {
        ParserError {
            summary: String::from(sum),
            parsertext: text,
            index: idx,
            explanation: None,
            caused_by: None,
        }
    }

    pub fn with_cause(mut self, e: Box<Error>) -> ParserError {
        self.caused_by = Some(e);
        self
    }

}

impl Error for ParserError {

    fn description(&self) -> &str {
        &self.summary[..]
    }

    fn cause(&self) -> Option<&Error> {
        self.caused_by.as_ref().map(|e| &**e)
    }

}

impl Debug for ParserError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "ParserError: {}\n\n", self.summary);

        if let Some(ref e) = self.explanation {
            write!(fmt, "{}\n\n", e);
        }

        write!(fmt, "On position {}\nin\n{}", self.index, self.parsertext);
        Ok(())
    }

}

impl Display for ParserError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "ParserError: {}", self.summary);

        if let Some(ref e) = self.explanation {
            write!(fmt, "\n\n{}", e);
        }

        Ok(())
    }

}

pub trait FileHeaderParser : Sized {
    fn read(&self, string: Option<String>) -> Result<FileHeaderData, ParserError>;
    fn write(&self, data: &FileHeaderData) -> Result<String, ParserError>;
}

type TextTpl = (Option<String>, Option<String>);

pub struct Parser<HP>
{
    headerp : HP,
}

impl<HP> Parser<HP> where
    HP: FileHeaderParser,
{

    pub fn new(headerp: HP) -> Parser<HP> {
        Parser {
            headerp: headerp,
        }
    }

    pub fn read(&self, s: String) -> Result<(FileHeaderData, String), ParserError>
    {
        debug!("Reading into internal datastructure: '{}'", s);
        let divided = self.divide_text(&s);

        if divided.is_err() {
            debug!("Error reading into internal datastructure");
            let mut p = ParserError::new("Dividing text failed",
                                         s, 0,
                    "Dividing text with divide_text() failed");
            return Err(p.with_cause(Box::new(divided.err().unwrap())));
        }

        let (header, data) = divided.ok().unwrap();
        debug!("Header = '{:?}'", header);
        debug!("Data   = '{:?}'", data);

        let h_parseres = try!(self.headerp.read(header));
        debug!("Success parsing header");

        Ok((h_parseres, data.unwrap_or(String::new())))
    }

    pub fn write(&self, tpl : (FileHeaderData, String)) -> Result<String, ParserError>
    {
        debug!("Parsing internal datastructure to String");
        let (header, data) = tpl;
        let h_text = try!(self.headerp.write(&header));
        debug!("Success translating header");

        let text = format!("---\n{}\n---\n{}", h_text, data);
        Ok(text)
    }

    fn divide_text(&self, text: &String) -> Result<TextTpl, ParserError> {
        let re = Regex::new(r"(?sm)^---$(.*)^---$(.*)").unwrap();

        debug!("Splitting: '{}'", text);
        debug!("   regex = {:?}", re);

        re.captures(text).map(|captures| {

            if captures.len() != 3 {
                debug!("Unexpected amount of captures");
                return Err(ParserError::new("Unexpected Regex output",
                                            text.clone(), 0,
                                            "The regex to divide text into header and content had an unexpected output."))
            }

            let header  = captures.at(1).map(|s| String::from(s));
            let content = captures.at(2).map(|s| String::from(s));

            debug!("Splitted, Header = '{:?}'", header.clone().unwrap_or("NONE".into()));
            debug!("Splitted, Data   = '{:?}'", content.clone().unwrap_or("NONE".into()));
            Ok((header, content))
        }).or_else(|| {
            debug!("Cannot capture from text");
            let e = ParserError::new("Cannot run regex on text",
                                     text.clone(), 0,
                                     "Cannot run regex on text to divide it into header and content.");
            Some(Err(e))
        }).unwrap()
    }

}

