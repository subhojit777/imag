use std::error::Error;
use std::result::Result as RResult;

use toml::Table;

pub mod error {
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt;
    use std::error::Error;
    use toml;

    #[derive(Clone)]
    pub enum ParserErrorKind {
        TOMLParserErrors,
        MissingMainSection,
    }

    pub struct ParserError {
        kind: ParserErrorKind,
        cause: Option<Box<Error>>,
    }

    impl ParserError {

        pub fn new(k: ParserErrorKind, cause: Option<Box<Error>>) -> ParserError {
            ParserError {
                kind: k,
                cause: cause,
            }
        }

    }

    impl Debug for ParserError {

        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            try!(write!(f, "{:?}", self.description()));
            Ok(())
        }

    }

    impl Display for ParserError {

        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            try!(write!(f, "{}", self.description()));
            Ok(())
        }

    }

    impl Error for ParserError {

        fn description(&self) -> &str {
            match self.kind {
                ParserErrorKind::MissingMainSection => "Missing main section",
                ParserErrorKind::TOMLParserErrors   => "Several TOML-Parser-Errors",
            }
        }

        fn cause(&self) -> Option<&Error> {
            self.cause.as_ref().map(|e| &**e)
        }

    }

}


use self::error::ParserErrorKind;
use self::error::ParserError;

/**
 * EntryHeader
 *
 * This is basically a wrapper around toml::Table which provides convenience to the user of the
 * librray.
 */
#[derive(Debug, Clone)]
pub struct EntryHeader {
    toml: Table,
}

pub type Result<V> = RResult<V, error::ParserError>;

/**
 * Wrapper type around file header (TOML) object
 */
impl EntryHeader {

    /**
     * Get a new header object with a already-filled toml table
     */
    pub fn new(toml: Table) -> EntryHeader {
        EntryHeader {
            toml: toml,
        }
    }

    /**
     * Get the table which lives in the background
     */
    pub fn toml(&self) -> &Table {
        &self.toml
    }

    pub fn parse(s: &str) -> Result<EntryHeader> {
        use toml::Parser;

        let mut parser = Parser::new(s);
        parser.parse()
            .ok_or(ParserError::new(ParserErrorKind::TOMLParserErrors, None))
            .map(|table| EntryHeader::new(table))
    }

}
