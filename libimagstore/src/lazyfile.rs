
use error::{StoreError, StoreErrorKind};
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};

pub enum LazyFile {
    Absent(PathBuf),
    File(File)
}

fn open_file<A: AsRef<Path>>(p: A) -> ::std::io::Result<File> {
    OpenOptions::new().write(true).read(true).open(p)
}

fn create_file<A: AsRef<Path>>(p: A) -> ::std::io::Result<File> {
    OpenOptions::new().write(true).read(true).create(true).open(p)
}

impl LazyFile {
    pub fn new(p: PathBuf) -> LazyFile {
        LazyFile::Absent(p)
    }
    pub fn new_with_file(f: File) -> LazyFile {
        LazyFile::File(f)
    }

    pub fn get_file(&mut self) -> Result<&File, StoreError> {
        let file = match *self {
            LazyFile::File(ref f) => return Ok(f),
            LazyFile::Absent(ref p) => {
                try!(open_file(p).map_err(|e| {
                    StoreError::new(StoreErrorKind::FileNotFound,
                                    Some(Box::new(e)))
                }))

            }
        };
        *self = LazyFile::File(file);
        if let LazyFile::File(ref f) = *self {
            return Ok(f);
        }
        unreachable!()
    }

    pub fn get_file_mut(&mut self) -> Result<&mut File, StoreError> {
        let file = match *self {
            LazyFile::File(ref mut f) => return Ok(f),
            LazyFile::Absent(ref p) => {
                try!(open_file(p).map_err(|e| {
                    StoreError::new(StoreErrorKind::FileNotFound,
                                    Some(Box::new(e)))
                }))
            }
        };
        *self = LazyFile::File(file);
        if let LazyFile::File(ref mut f) = *self {
            return Ok(f);
        }
        unreachable!()
    }

    pub fn create_file(&mut self) -> Result<&mut File, StoreError> {
        let file = match *self {
            LazyFile::File(ref mut f) => return Ok(f),
            LazyFile::Absent(ref p) => {
                try!(create_file(p).map_err(|e| {
                    StoreError::new(StoreErrorKind::FileNotFound,
                                    Some(Box::new(e)))
                }))
            }
        };
        *self = LazyFile::File(file);
        if let LazyFile::File(ref mut f) = *self {
            return Ok(f);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::LazyFile;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use std::fs::File;

    #[test]
    fn lazy_file() {
        let path = PathBuf::from("/tmp/test");
        let mut lf = LazyFile::new(path);

        write!(lf.create_file().unwrap(), "Hello World").unwrap();
    }

    #[test]
    fn lazy_file_with_file() {
        let path = PathBuf::from("/tmp/test2");
        let mut lf = LazyFile::new_with_file(File::create(path).unwrap());
        let mut file = lf.get_file_mut().unwrap();

        write!(file, "Hello World").unwrap();
        file.sync_all().unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        assert_eq!(s, "Hello World");
    }
}
