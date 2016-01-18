
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
    use tempdir::TempDir;

    fn get_dir() -> TempDir {
        TempDir::new("test-image").unwrap()
    }

    #[test]
    fn lazy_file() {
        let dir = get_dir();
        let mut path = PathBuf::from(dir.path());
        path.set_file_name("test1");
        let mut lf = LazyFile::new(path);

        write!(lf.create_file().unwrap(), "Hello World").unwrap();
        dir.close().unwrap();
    }

    #[test]
    fn lazy_file_with_file() {
        let dir = get_dir();
        let mut path = PathBuf::from(dir.path());
        path.set_file_name("test2");

        {
            let mut lf = LazyFile::new(path.clone());
            let mut file = lf.create_file().unwrap();

            file.write(b"Hello World").unwrap();
            file.sync_all().unwrap();
        }

        {
            let mut lf = LazyFile::new(path);
            let mut file = lf.create_file().unwrap();

            let mut s = Vec::new();
            file.read_to_end(&mut s).unwrap();
            assert_eq!(s, "Hello World".to_string().into_bytes());
        }

        dir.close().unwrap();
    }
}
