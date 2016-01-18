use std::error::Error;
use std::result::Result as RResult;

use toml::{Table, Value};

pub mod error {
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt;
    use std::error::Error;
    use toml;

    #[derive(Clone)]
    pub enum ParserErrorKind {
        TOMLParserErrors,
        MissingMainSection,
        MissingVersionInfo,
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
                ParserErrorKind::TOMLParserErrors   => "Several TOML-Parser-Errors",
                ParserErrorKind::MissingMainSection => "Missing main section",
                ParserErrorKind::MissingVersionInfo => "Missing version information in main section",
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
            .and_then(|t| verify_header_consistency(t))
            .map(|t| EntryHeader::new(t))
    }

}

fn verify_header_consistency(t: Table) -> Result<Table> {
    if !has_main_section(&t) {
        Err(ParserError::new(ParserErrorKind::MissingMainSection, None))
    } else if !has_imag_version_in_main_section(&t) {
        Err(ParserError::new(ParserErrorKind::MissingVersionInfo, None))
    } else {
        Ok(t)
    }
}

fn has_main_section(t: &Table) -> bool {
    t.contains_key("imag") &&
        match t.get("imag") {
            Some(&Value::Table(_)) => true,
            Some(_)                => false,
            None                   => false,
        }
}

fn has_imag_version_in_main_section(t: &Table) -> bool {
    match t.get("imag").unwrap() {
        &Value::Table(ref sec) => {
            sec.get("version")
                .and_then(|v| {
                    match v {
                        &Value::String(_) => Some(true),
                        _                 => Some(false),
                    }
                })
                .unwrap_or(false)
        }
        _                  => false,
    }
}


#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use toml::Value;

    #[test]
    fn test_imag_section() {
        use super::has_main_section;

        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Table(BTreeMap::new()));

        assert!(has_main_section(&map));
    }

    #[test]
    fn test_imag_invalid_section_type() {
        use super::has_main_section;

        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Boolean(false));

        assert!(!has_main_section(&map));
    }

    #[test]
    fn test_imag_abscent_main_section() {
        use super::has_main_section;

        let mut map = BTreeMap::new();
        map.insert("not_imag".into(), Value::Boolean(false));

        assert!(!has_main_section(&map));
    }

    #[test]
    fn test_main_section_without_version() {
        use super::has_imag_version_in_main_section;

        let mut map = BTreeMap::new();
        map.insert("imag".into(), Value::Table(BTreeMap::new()));

        assert!(!has_imag_version_in_main_section(&map));
    }

    #[test]
    fn test_main_section_with_version() {
        use super::has_imag_version_in_main_section;

        let mut map = BTreeMap::new();
        let mut sub = BTreeMap::new();
        sub.insert("version".into(), Value::String("0.0.0".into()));
        map.insert("imag".into(), Value::Table(sub));

        assert!(has_imag_version_in_main_section(&map));
    }

    #[test]
    fn test_main_section_with_version_in_wrong_type() {
        use super::has_imag_version_in_main_section;

        let mut map = BTreeMap::new();
        let mut sub = BTreeMap::new();
        sub.insert("version".into(), Value::Boolean(false));
        map.insert("imag".into(), Value::Table(sub));

        assert!(!has_imag_version_in_main_section(&map));
    }

}

