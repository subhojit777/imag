pub use self::fs::LazyFile;

#[cfg(test)]
mod fs {
    use error::StoreError as SE;
    use std::io::Cursor;
    use std::path::PathBuf;

    use std::collections::HashMap;
    use std::sync::Mutex;

    lazy_static! {
        static ref MAP: Mutex<HashMap<PathBuf, Cursor<Vec<u8>>>> = {
            Mutex::new(HashMap::new())
        };
    }

    /// `LazyFile` type, this is the Test version!
    ///
    /// A lazy file is either absent, but a path to it is available, or it is present.
    #[derive(Debug)]
    pub enum LazyFile {
        Absent(PathBuf),
    }

    impl LazyFile {

        /**
         * Get the mutable file behind a LazyFile object
         */
        pub fn get_file_content(&mut self) -> Result<Cursor<Vec<u8>>, SE> {
            debug!("Getting lazy file: {:?}", self);
            match *self {
                LazyFile::Absent(ref f) => {
                    let map = MAP.lock().unwrap();
                    return Ok(map.get(f).unwrap().clone());
                },
            };
        }

        pub fn write_file_content(&mut self, buf: &[u8]) -> Result<(), SE> {
            match *self {
                LazyFile::Absent(ref f) => {
                    let mut map = MAP.lock().unwrap();
                    if let Some(ref mut cur) = map.get_mut(f) {
                        let mut vec = cur.get_mut();
                        vec.clear();
                        vec.extend_from_slice(buf);
                        return Ok(());
                    }
                    let vec = Vec::from(buf);
                    map.insert(f.clone(), Cursor::new(vec));
                    return Ok(());
                },
            };
        }
    }
}

#[cfg(not(test))]
mod fs {
    use error::{MapErrInto, StoreError as SE, StoreErrorKind as SEK};
    use std::io::{Seek, SeekFrom, Read};
    use std::path::{Path, PathBuf};
    use std::fs::{File, OpenOptions, create_dir_all};

    /// `LazyFile` type
    ///
    /// A lazy file is either absent, but a path to it is available, or it is present.
    #[derive(Debug)]
    pub enum LazyFile {
        Absent(PathBuf),
        File(File)
    }

    fn open_file<A: AsRef<Path>>(p: A) -> ::std::io::Result<File> {
        OpenOptions::new().write(true).read(true).open(p)
    }

    fn create_file<A: AsRef<Path>>(p: A) -> ::std::io::Result<File> {
        if let Some(parent) = p.as_ref().parent() {
            debug!("Implicitely creating directory: {:?}", parent);
            if let Err(e) = create_dir_all(parent) {
                return Err(e);
            }
        }
        OpenOptions::new().write(true).read(true).create(true).open(p)
    }

    impl LazyFile {

        /**
         * Get the content behind this file
         */
        pub fn get_file_content(&mut self) -> Result<&mut Read, SE> {
            debug!("Getting lazy file: {:?}", self);
            let file = match *self {
                LazyFile::File(ref mut f) => return {
                    // We seek to the beginning of the file since we expect each
                    // access to the file to be in a different context
                    try!(f.seek(SeekFrom::Start(0))
                        .map_err_into(SEK::FileNotSeeked));
                    Ok(f)
                },
                LazyFile::Absent(ref p) => try!(open_file(p).map_err_into(SEK::FileNotFound)),
            };
            *self = LazyFile::File(file);
            if let LazyFile::File(ref mut f) = *self {
                return Ok(f);
            }
            unreachable!()
        }

        /**
         * Write the content of this file
         */
        pub fn write_file_content(&mut self, buf: &[u8]) -> Result<(), SE> {
            use std::io::Write;
            let file = match *self {
                LazyFile::File(ref mut f) => return {
                    // We seek to the beginning of the file since we expect each
                    // access to the file to be in a different context
                    try!(f.seek(SeekFrom::Start(0))
                        .map_err_into(SEK::FileNotCreated));
                    f.write_all(buf).map_err_into(SEK::FileNotWritten)
                },
                LazyFile::Absent(ref p) => try!(create_file(p).map_err_into(SEK::FileNotCreated)),
            };
            *self = LazyFile::File(file);
            if let LazyFile::File(ref mut f) = *self {
                return f.write_all(buf).map_err_into(SEK::FileNotWritten);
            }
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test {
    // use super::LazyFile;
    // use std::io::{Read, Write};
    // use std::path::PathBuf;
    // use tempdir::TempDir;

    // fn get_dir() -> TempDir {
    //     TempDir::new("test-image").unwrap()
    // }

    // #[test]
    // fn lazy_file() {
    //     let dir = get_dir();
    //     let mut path = PathBuf::from(dir.path());
    //     path.set_file_name("test1");
    //     let mut lf = LazyFile::Absent(path);

    //     write!(lf.create_file().unwrap(), "Hello World").unwrap();
    //     dir.close().unwrap();
    // }

    // #[test]
    // fn lazy_file_with_file() {
    //     let dir = get_dir();
    //     let mut path = PathBuf::from(dir.path());
    //     path.set_file_name("test2");
    //     let mut lf = LazyFile::Absent(path.clone());

    //     {
    //         let mut file = lf.create_file().unwrap();

    //         file.write(b"Hello World").unwrap();
    //         file.sync_all().unwrap();
    //     }

    //     {
    //         let mut file = lf.get_file_mut().unwrap();
    //         let mut s = Vec::new();
    //         file.read_to_end(&mut s).unwrap();
    //         assert_eq!(s, "Hello World".to_string().into_bytes());
    //     }

    //     dir.close().unwrap();
    // }
}
