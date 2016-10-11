//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

pub use self::fs::FileAbstraction;

// TODO:
// This whole thing can be written better with a trait based mechanism that is embedded into the
// store. However it would mean rewriting most things to be generic which can be a pain in the ass.

#[cfg(test)]
mod fs {
    use error::StoreError as SE;
    use error::StoreErrorKind as SEK;
    use std::io::Cursor;
    use std::path::PathBuf;

    use libimagerror::into::IntoError;

    use std::collections::HashMap;
    use std::sync::Mutex;

    lazy_static! {
        static ref MAP: Mutex<HashMap<PathBuf, Cursor<Vec<u8>>>> = {
            Mutex::new(HashMap::new())
        };
    }

    /// `FileAbstraction` type, this is the Test version!
    ///
    /// A lazy file is either absent, but a path to it is available, or it is present.
    #[derive(Debug)]
    pub enum FileAbstraction {
        Absent(PathBuf),
    }

    impl FileAbstraction {

        /**
         * Get the mutable file behind a FileAbstraction object
         */
        pub fn get_file_content(&mut self) -> Result<Cursor<Vec<u8>>, SE> {
            debug!("Getting lazy file: {:?}", self);
            match *self {
                FileAbstraction::Absent(ref f) => {
                    let map = MAP.lock().unwrap();
                    return map.get(f).cloned().ok_or(SEK::FileNotFound.into_error());
                },
            };
        }

        pub fn write_file_content(&mut self, buf: &[u8]) -> Result<(), SE> {
            match *self {
                FileAbstraction::Absent(ref f) => {
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

        pub fn remove_file(path: &PathBuf) -> Result<(), SE> {
            MAP.lock().unwrap().remove(path);
            Ok(())
        }

        pub fn copy(from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            let mut map = MAP.lock().unwrap();
            let a = map.get(from).unwrap().clone();
            map.insert(to.clone(), a);
            Ok(())
        }

        pub fn rename(from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            let mut map = MAP.lock().unwrap();
            let a = map.get(from).unwrap().clone();
            map.insert(to.clone(), a);
            Ok(())
        }

        pub fn create_dir_all(_: &PathBuf) -> Result<(), SE> {
            Ok(())
        }
    }
}

#[cfg(not(test))]
mod fs {
    use error::{MapErrInto, StoreError as SE, StoreErrorKind as SEK};
    use std::io::{Seek, SeekFrom, Read};
    use std::path::{Path, PathBuf};
    use std::fs::{File, OpenOptions, create_dir_all, remove_file, copy, rename};

    /// `FileAbstraction` type
    ///
    /// A lazy file is either absent, but a path to it is available, or it is present.
    #[derive(Debug)]
    pub enum FileAbstraction {
        Absent(PathBuf),
        File(File, PathBuf)
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

    impl FileAbstraction {

        /**
         * Get the content behind this file
         */
        pub fn get_file_content(&mut self) -> Result<&mut Read, SE> {
            debug!("Getting lazy file: {:?}", self);
            let (file, path) = match *self {
                FileAbstraction::File(ref mut f, _) => return {
                    // We seek to the beginning of the file since we expect each
                    // access to the file to be in a different context
                    try!(f.seek(SeekFrom::Start(0))
                        .map_err_into(SEK::FileNotSeeked));
                    Ok(f)
                },
                FileAbstraction::Absent(ref p) => (try!(open_file(p).map_err_into(SEK::FileNotFound)),
                                            p.clone()),
            };
            *self = FileAbstraction::File(file, path);
            if let FileAbstraction::File(ref mut f, _) = *self {
                return Ok(f);
            }
            unreachable!()
        }

        /**
         * Write the content of this file
         */
        pub fn write_file_content(&mut self, buf: &[u8]) -> Result<(), SE> {
            use std::io::Write;
            let (file, path) = match *self {
                FileAbstraction::File(ref mut f, _) => return {
                    // We seek to the beginning of the file since we expect each
                    // access to the file to be in a different context
                    try!(f.seek(SeekFrom::Start(0))
                        .map_err_into(SEK::FileNotCreated));
                    f.write_all(buf).map_err_into(SEK::FileNotWritten)
                },
                FileAbstraction::Absent(ref p) => (try!(create_file(p).map_err_into(SEK::FileNotCreated)),
                                            p.clone()),
            };
            *self = FileAbstraction::File(file, path);
            if let FileAbstraction::File(ref mut f, _) = *self {
                return f.write_all(buf).map_err_into(SEK::FileNotWritten);
            }
            unreachable!();
        }

        pub fn remove_file(path: &PathBuf) -> Result<(), SE> {
            remove_file(path).map_err_into(SEK::FileNotRemoved)
        }

        pub fn copy(from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            copy(from, to).map_err_into(SEK::FileNotCopied).map(|_| ())
        }

        pub fn rename(from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            rename(from, to).map_err_into(SEK::FileNotRenamed)
        }

        pub fn create_dir_all(path: &PathBuf) -> Result<(), SE> {
            create_dir_all(path).map_err_into(SEK::DirNotCreated)
        }
    }
}

#[cfg(test)]
mod test {
    use super::FileAbstraction;
    use std::io::Read;
    use std::path::PathBuf;

    #[test]
    fn lazy_file() {
        let mut path = PathBuf::from("/tests");
        path.set_file_name("test1");
        let mut lf = FileAbstraction::Absent(path);
        lf.write_file_content(b"Hello World").unwrap();
        let mut bah = Vec::new();
        lf.get_file_content().unwrap().read_to_end(&mut bah).unwrap();
        assert_eq!(bah, b"Hello World");
    }

}
