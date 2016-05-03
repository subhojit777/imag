
use error::{StoreError, StoreErrorKind};
use std::io::{Seek, SeekFrom};
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
     * Get the mutable file behind a LazyFile object
     */
    pub fn get_file_mut(&mut self) -> Result<&mut File, StoreError> {
        debug!("Getting lazy file: {:?}", self);
        let file = match *self {
            LazyFile::File(ref mut f) => return {
                // We seek to the beginning of the file since we expect each
                // access to the file to be in a different context
                f.seek(SeekFrom::Start(0))
                    .map_err(|e| StoreError::new(StoreErrorKind::FileNotCreated, Some(Box::new(e))))
                    .map(|_| f)
            },
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

    /**
     * Create a file out of this LazyFile object
     */
    pub fn create_file(&mut self) -> Result<&mut File, StoreError> {
        debug!("Creating lazy file: {:?}", self);
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
    use tempdir::TempDir;

    fn get_dir() -> TempDir {
        TempDir::new("test-image").unwrap()
    }

    #[test]
    fn lazy_file() {
        let dir = get_dir();
        let mut path = PathBuf::from(dir.path());
        path.set_file_name("test1");
        let mut lf = LazyFile::Absent(path);

        write!(lf.create_file().unwrap(), "Hello World").unwrap();
        dir.close().unwrap();
    }

    #[test]
    fn lazy_file_with_file() {
        let dir = get_dir();
        let mut path = PathBuf::from(dir.path());
        path.set_file_name("test2");
        let mut lf = LazyFile::Absent(path.clone());

        {
            let mut file = lf.create_file().unwrap();

            file.write(b"Hello World").unwrap();
            file.sync_all().unwrap();
        }

        {
            let mut file = lf.get_file_mut().unwrap();
            let mut s = Vec::new();
            file.read_to_end(&mut s).unwrap();
            assert_eq!(s, "Hello World".to_string().into_bytes());
        }

        dir.close().unwrap();
    }
}
