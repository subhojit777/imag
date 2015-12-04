use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::result::Result;
use std::path::{Path, PathBuf};
use std::convert::From;
use std::convert::Into;

use regex::Regex;

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
        let s = match self {
            FileIDType::UUID => "UUID",
            FileIDType::NONE => "",
        };

        String::from(s)
    }

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

#[derive(Clone)]
pub struct FileID {
    id: Option<String>,
    id_type: FileIDType,
}

impl FileID {

    pub fn new(id_type: FileIDType, id: String) -> FileID {
        FileID {
            id: Some(id),
            id_type: id_type,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.id.is_some()
    }

    pub fn get_type(&self) -> FileIDType {
        self.id_type.clone()
    }

    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

}

impl Debug for FileID {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "FileID[{:?}]: {:?}",
               self.id_type,
               self.id);
        Ok(())
    }

}

impl Display for FileID {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "FileID[{:?}]: {:?}",
               self.id_type,
               self.id);
        Ok(())
    }

}

impl Into<String> for FileID {

    fn into(self) -> String {
        if let Some(id) = self.id {
            id.clone()
        } else {
            String::from("INVALID")
        }
    }

}

impl Into<FileIDType> for FileID {

    fn into(self) -> FileIDType {
        self.id_type.clone()
    }
}

impl From<String> for FileID {

    fn from(s: String) -> FileID {
        FileID::from(&s)
    }

}

impl<'a> From<&'a String> for FileID {

    fn from(string: &'a String) -> FileID {
        // we assume that it is an path
        let regex = Regex::new(r"([:alnum:]*)-([:upper:]*)-([A-Za-z0-9-_]*)\.(.*)").unwrap();
        let s = string.split("/").last().unwrap_or("");

        debug!("Regex build: {:?}", regex);
        debug!("Matching string: '{}'", s);
        regex.captures(s).and_then(|capts| {
            // first one is the whole string, index 1-N are the matches.
            if capts.len() != 5 {
                debug!("Matches, but not expected number of groups");
                return None;
            }
            debug!("Matches: {}", capts.len());

            let modname     = capts.at(1).unwrap();
            let hashname    = capts.at(2).unwrap();
            let mut hash    = capts.at(3).unwrap();

            debug!("Destructure FilePath to ID:");
            debug!("                  FilePath: {:?}", s);
            debug!("               Module Name: {:?}", modname);
            debug!("                 Hash Name: {:?}", hashname);
            debug!("                      Hash: {:?}", hash);

            let idtype = FileIDType::from(hashname);
            match idtype {
                FileIDType::NONE => hash = "INVALID",
                _ => {},
            }

            Some(FileID::new(idtype, String::from(hash)))
        }).unwrap_or({
            debug!("Did not match");
            debug!("It is no path, actually. So we assume it is an ID already");
            FileID {
                id_type: FileIDType::NONE,
                id: Some(string.clone()),
            }
        })
    }

}

impl From<PathBuf> for FileID {

    fn from(s: PathBuf) -> FileID {
        unimplemented!()
    }

}

impl<'a> From<&'a PathBuf> for FileID {

    fn from(pb: &'a PathBuf) -> FileID {
        let s = pb.to_str().unwrap_or("");
        FileID::from(String::from(s))
    }

}

pub struct FileIDError {
    summary: String,
    descrip: String,
}

impl FileIDError {

    pub fn new(s: String, d: String) -> FileIDError {
        FileIDError {
            summary: s,
            descrip: d,
        }
    }

}

impl<'a> Error for FileIDError {

    fn description(&self) -> &str {
        &self.summary[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl<'a> Debug for FileIDError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "FileIDError: '{}'\n{}", self.summary, self.descrip);
        Ok(())
    }

}

impl<'a> Display for FileIDError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "FileIDError: '{}'", self.summary);
        Ok(())
    }

}

pub type FileIDResult = Result<FileID, FileIDError>;

#[cfg(test)]
mod test {

    use super::{FileID, FileIDType};

    #[test]
    fn file_id_from_string() {
        setup_logger();

        let s1 = String::from("/home/user/testmodule-UUID-some-id.imag");
        let s2 = String::from("/home/user/testmodule-UUID-some-id.extension.imag");
        let s3 = String::from("/home/user/testmodule-NOHASH-some-id.imag");

        let id1 = FileID::from(s1);
        let id2 = FileID::from(s2);
        let id3 = FileID::from(s3);

        println!("Id 1 : {:?}", id1);
        println!("Id 2 : {:?}", id2);
        println!("Id 3 : {:?}", id3);

        assert_eq!(FileIDType::UUID, id1.get_type());
        assert_eq!(FileIDType::UUID, id2.get_type());
        assert_eq!(FileIDType::NONE, id3.get_type());

        let f1 : String = id1.into();
        let f2 : String = id2.into();
        let f3 : String = id3.into();

        assert_eq!(String::from("some-id"), f1);
        assert_eq!(String::from("some-id"), f2);
        assert_eq!(String::from("INVALID"), f3);
    }

    fn setup_logger() {
        extern crate log;
        use log::{LogLevelFilter, set_logger};
        use runtime::ImagLogger;

        log::set_logger(|max_log_lvl| {
            let lvl = LogLevelFilter::Debug;
            max_log_lvl.set(lvl);
            Box::new(ImagLogger::new(lvl.to_log_level().unwrap()))
        });
        debug!("Init logger for test");
    }

}

