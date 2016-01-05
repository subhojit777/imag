use std::fmt::{Display, Formatter};
use std::fmt;
use std::convert::{From, Into};
use std::str::FromStr;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
/**
 * File ID type
 *
 * Currently only UUID is available. Maybe this will be the only type available at all.
 */
pub enum FileIDType {
    UUID,
}

pub enum FileIDTypeParseError {
    UnknownType
}

impl FromStr for FileIDType {
    type Err = FileIDTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UUID" => Ok(FileIDType::UUID),
            _ => Err(FileIDTypeParseError::UnknownType)
        }
    }
}

impl Into<String> for FileIDType {

    fn into(self) -> String {
        match self {
            FileIDType::UUID    => String::from("UUID"),
        }
    }
}

impl Display for FileIDType {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            &FileIDType::UUID => try!(write!(fmt, "UUID")),
        }
        Ok(())
    }
}

