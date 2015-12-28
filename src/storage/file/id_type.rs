use std::convert::{From, Into};
use std::error::Error;
use std::str::FromStr;
use std::hash::Hash;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
// #[derive(Display)]
#[derive(Hash)]
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

