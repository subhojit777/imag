use std::convert::{From, Into};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::hash::Hash;
use std::path::PathBuf;
use std::result::Result;
use std::str::FromStr;

use regex::Regex;

use storage::file::id_type::FileIDType;
use storage::file::hash::FileHash;

#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct FileID {
    id:         FileHash,
    id_type:    FileIDType,
}

impl FileID {

    pub fn new(id_type: FileIDType, id: FileHash) -> FileID {
        FileID {
            id: id,
            id_type: id_type,
        }
    }

    pub fn get_type(&self) -> FileIDType {
        self.id_type.clone()
    }

    pub fn get_id(&self) -> FileHash {
        self.id.clone()
    }

    pub fn parse(string: &String) -> Option<FileID> {
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

            FileIDType::from_str(hashname).map(|idtype| {
                Some(FileID {
                    id: FileHash::from(hash),
                    id_type: idtype,
                })
            }).ok()
        }).unwrap_or({
            debug!("Did not match");
            debug!("It is no path, actually. So we assume it is an ID already");
            None
        })
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
        let typestr : String = self.id_type.into();
        let idstr   : String = self.id.into();
        typestr + &idstr[..]
    }
}

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

