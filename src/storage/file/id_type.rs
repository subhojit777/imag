use std::convert::{From, Into};
use std::error::Error;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
// #[derive(Display)]
pub enum FileIDType {
    UUID,
}

impl FileIDType {

    fn parse(s: &str) -> Option<FileIDType> {
        unimplemented!()
    }

}

impl Into<String> for FileIDType {

    fn into(self) -> String {
        match self {
            FileIDType::UUID    => String::from("UUID"),
        }
    }
}

