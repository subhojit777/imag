use std::convert::{From, Into};
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::result::Result;
use std::str::FromStr;

use regex::Regex;

use storage::file::id_type::FileIDType;
use storage::file::hash::FileHash;

#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
/**
 * FileID
 *
 * A FileID contains of two parts: The ID type and the Hash. For example the FileID
 *
 *  UUID-235-1215-1212
 *
 * has a type ("UUID") and a Hash ("235-1215-1212").
 */
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

    /**
     * Get the type of the FileID
     */
    pub fn get_type(&self) -> FileIDType {
        self.id_type.clone()
    }

    /**
     * Get the Hash of the FileID
     */
    pub fn get_id(&self) -> FileHash {
        self.id.clone()
    }

    /**
     * Parse a String into a FileID, if possible
     */
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
                debug!("ID type = {:?}", idtype);
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
        typestr + "-" + &idstr[..]
    }
}

#[cfg(test)]
mod test {

    use storage::file::id::FileID;
    use storage::file::id_type::FileIDType;

    #[test]
    fn file_id_from_string() {
        setup_logger();

        let s1 = String::from("/home/user/testmodule-UUID-some-id.imag");
        let s2 = String::from("/home/user/testmodule-UUID-some-id.extension.imag");
        let s3 = String::from("/home/user/testmodule-NOHASH-some-id.imag");

        let id1 = FileID::parse(&s1).unwrap();
        let id2 = FileID::parse(&s2).unwrap();
        assert!(FileID::parse(&s3).is_none());

        println!("Id 1 : {:?}", id1);
        println!("Id 2 : {:?}", id2);

        assert_eq!(FileIDType::UUID, id1.get_type());
        assert_eq!(FileIDType::UUID, id2.get_type());

        let h1 : String = id1.get_id().into();
        let h2 : String = id2.get_id().into();

        assert_eq!(String::from("some-id"), h1);
        assert_eq!(String::from("some-id"), h2);

        let f1 : String = id1.into();
        let f2 : String = id2.into();

        assert_eq!(String::from("UUID-some-id"), f1);
        assert_eq!(String::from("UUID-some-id"), f2);
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

