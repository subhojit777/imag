use std::convert::{From, Into};
use std::error::Error;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
// #[derive(Display)]
pub enum FileIDType {
    NONE,
    UUID,
}

impl Into<String> for FileIDType {

    fn into(self) -> String {
        into_string(&self)
    }

}

impl<'a> From<&'a FileIDType> for String {

    fn from(t: &'a FileIDType) -> String {
        into_string(t)
    }

}

fn into_string(t: &FileIDType) -> String {
    let s = match t {
        &FileIDType::UUID => "UUID",
        &FileIDType::NONE => "",
    };

    String::from(s)
}

impl<'a> From<&'a str> for FileIDType {

    fn from(s: &'a str) -> FileIDType {
        match s {
            "UUID"  => FileIDType::UUID,
            _       => FileIDType::NONE,
        }
    }

}

impl From<String> for FileIDType {

    fn from(s: String) -> FileIDType {
        FileIDType::from(&s[..])
    }

}

