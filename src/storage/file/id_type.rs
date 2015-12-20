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

