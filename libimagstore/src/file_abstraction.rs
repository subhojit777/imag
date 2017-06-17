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

//! The filesystem abstraction code
//!
//! # Problem
//!
//! First, we had a compiletime backend for the store. This means that the actual filesystem
//! operations were compiled into the store either as real filesystem operations (in a normal debug
//! or release build) but as a in-memory variant in the 'test' case.
//! So tests did not hit the filesystem when running.
//! This gave us us the possibility to run tests concurrently with multiple
//! stores that did not interfere with eachother.
//!
//! This approach worked perfectly well until we started to test not the
//! store itself but crates that depend on the store implementation.
//! When running tests in a crate that depends on the store, the store
//! itself was compiled with the filesystem-hitting-backend.
//! This was problematic, as tests could not be implemented without hitting
//! the filesystem.
//!
//! Hence we implemented this.
//!
//! # Implementation
//!
//! The filesystem is abstracted via a trait `FileAbstraction` which
//! contains the essential functions for working with the filesystem.
//!
//! Two implementations are provided in the code:
//!
//! * FSFileAbstraction
//! * InMemoryFileAbstraction
//!
//! whereas the first actually works with the filesystem and the latter
//! works with an in-memory HashMap that is used as filesystem.
//!
//! Further, the trait `FileAbstractionInstance` was introduced for
//! functions which are executed on actual instances of content from the
//! filesystem, which was previousely tied into the general abstraction
//! mechanism.
//!
//! So, the `FileAbstraction` trait is for working with the filesystem, the
//! `FileAbstractionInstance` trait is for working with instances of content
//! from the filesystem (speak: actual Files).
//!
//! In case of the `FSFileAbstractionInstance`, which is the implementation
//! of the `FileAbstractionInstance` for the actual filesystem-hitting code,
//! the underlying resource is managed like with the old code before.
//! The `InMemoryFileAbstractionInstance` implementation is corrosponding to
//! the `InMemoryFileAbstraction` implementation - for the in-memory
//! "filesystem".
//!
//! The implementation of the `get_file_content()` function had to be
//! changed to return a `String` rather than a `&mut Read` because of
//! lifetime issues.
//! This change is store-internally and the API of the store itself was not
//! affected.
//!

use std::path::PathBuf;
use std::fmt::Debug;

use error::StoreError as SE;

pub use self::fs::FSFileAbstraction;
pub use self::fs::FSFileAbstractionInstance;
pub use self::inmemory::InMemoryFileAbstraction;
pub use self::inmemory::InMemoryFileAbstractionInstance;

/// An abstraction trait over filesystem actions
pub trait FileAbstraction : Debug {
    fn remove_file(&self, path: &PathBuf) -> Result<(), SE>;
    fn copy(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE>;
    fn rename(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE>;
    fn create_dir_all(&self, _: &PathBuf) -> Result<(), SE>;

    fn new_instance(&self, p: PathBuf) -> Box<FileAbstractionInstance>;
}

/// An abstraction trait over actions on files
pub trait FileAbstractionInstance : Debug {
    fn get_file_content(&mut self) -> Result<String, SE>;
    fn write_file_content(&mut self, buf: &[u8]) -> Result<(), SE>;
}

mod fs {
    use std::fs::{File, OpenOptions, create_dir_all, remove_file, copy, rename};
    use std::io::{Seek, SeekFrom, Read};
    use std::path::{Path, PathBuf};

    use error::{MapErrInto, StoreError as SE, StoreErrorKind as SEK};

    use super::FileAbstraction;
    use super::FileAbstractionInstance;

    #[derive(Debug)]
    pub enum FSFileAbstractionInstance {
        Absent(PathBuf),
        File(File, PathBuf)
    }

    impl FileAbstractionInstance for FSFileAbstractionInstance {

        /**
         * Get the content behind this file
         */
        fn get_file_content(&mut self) -> Result<String, SE> {
            debug!("Getting lazy file: {:?}", self);
            let (file, path) = match *self {
                FSFileAbstractionInstance::File(ref mut f, _) => return {
                    // We seek to the beginning of the file since we expect each
                    // access to the file to be in a different context
                    try!(f.seek(SeekFrom::Start(0))
                        .map_err_into(SEK::FileNotSeeked));

                    let mut s = String::new();
                    f.read_to_string(&mut s)
                        .map_err_into(SEK::IoError)
                        .map(|_| s)
                },
                FSFileAbstractionInstance::Absent(ref p) =>
                    (try!(open_file(p).map_err_into(SEK::FileNotFound)), p.clone()),
            };
            *self = FSFileAbstractionInstance::File(file, path);
            if let FSFileAbstractionInstance::File(ref mut f, _) = *self {
                let mut s = String::new();
                f.read_to_string(&mut s)
                    .map_err_into(SEK::IoError)
                    .map(|_| s)
            } else {
                unreachable!()
            }
        }

        /**
         * Write the content of this file
         */
        fn write_file_content(&mut self, buf: &[u8]) -> Result<(), SE> {
            use std::io::Write;
            let (file, path) = match *self {
                FSFileAbstractionInstance::File(ref mut f, _) => return {
                    // We seek to the beginning of the file since we expect each
                    // access to the file to be in a different context
                    try!(f.seek(SeekFrom::Start(0))
                        .map_err_into(SEK::FileNotCreated));
                    f.write_all(buf).map_err_into(SEK::FileNotWritten)
                },
                FSFileAbstractionInstance::Absent(ref p) =>
                    (try!(create_file(p).map_err_into(SEK::FileNotCreated)), p.clone()),
            };
            *self = FSFileAbstractionInstance::File(file, path);
            if let FSFileAbstractionInstance::File(ref mut f, _) = *self {
                return f.write_all(buf).map_err_into(SEK::FileNotWritten);
            }
            unreachable!();
        }

    }

    /// `FSFileAbstraction` state type
    ///
    /// A lazy file is either absent, but a path to it is available, or it is present.
    #[derive(Debug)]
    pub struct FSFileAbstraction {
    }

    impl FSFileAbstraction {
        pub fn new() -> FSFileAbstraction {
            FSFileAbstraction { }
        }
    }

    impl FileAbstraction for FSFileAbstraction {

        fn remove_file(&self, path: &PathBuf) -> Result<(), SE> {
            remove_file(path).map_err_into(SEK::FileNotRemoved)
        }

        fn copy(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            copy(from, to).map_err_into(SEK::FileNotCopied).map(|_| ())
        }

        fn rename(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            rename(from, to).map_err_into(SEK::FileNotRenamed)
        }

        fn create_dir_all(&self, path: &PathBuf) -> Result<(), SE> {
            create_dir_all(path).map_err_into(SEK::DirNotCreated)
        }

        fn new_instance(&self, p: PathBuf) -> Box<FileAbstractionInstance> {
            Box::new(FSFileAbstractionInstance::Absent(p))
        }
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

}

mod inmemory {
    use error::StoreError as SE;
    use error::StoreErrorKind as SEK;
    use std::io::Read;
    use std::io::Cursor;
    use std::path::PathBuf;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::cell::RefCell;
    use std::sync::Arc;

    use libimagerror::into::IntoError;

    use super::FileAbstraction;
    use super::FileAbstractionInstance;
    use error::MapErrInto;

    type Backend = Arc<Mutex<RefCell<HashMap<PathBuf, Cursor<Vec<u8>>>>>>;

    /// `FileAbstraction` type, this is the Test version!
    ///
    /// A lazy file is either absent, but a path to it is available, or it is present.
    #[derive(Debug)]
    pub struct InMemoryFileAbstractionInstance {
        fs_abstraction: Backend,
        absent_path: PathBuf,
    }

    impl InMemoryFileAbstractionInstance {

        pub fn new(fs: Backend, pb: PathBuf) -> InMemoryFileAbstractionInstance {
            InMemoryFileAbstractionInstance {
                fs_abstraction: fs,
                absent_path: pb
            }
        }

    }

    impl FileAbstractionInstance for InMemoryFileAbstractionInstance {

        /**
         * Get the mutable file behind a InMemoryFileAbstraction object
         */
        fn get_file_content(&mut self) -> Result<String, SE> {
            debug!("Getting lazy file: {:?}", self);

            let p = self.absent_path.clone();
            match self.fs_abstraction.lock() {
                Ok(mut mtx) => {
                    mtx.get_mut()
                        .get_mut(&p)
                        .ok_or(SEK::FileNotFound.into_error())
                        .and_then(|t| {
                            let mut s = String::new();
                            t.read_to_string(&mut s)
                                .map_err_into(SEK::IoError)
                                .map(|_| s)
                        })
                }

                Err(_) => Err(SEK::LockError.into_error())
            }
        }

        fn write_file_content(&mut self, buf: &[u8]) -> Result<(), SE> {
            match *self {
                InMemoryFileAbstractionInstance { ref absent_path, .. } => {
                    let mut mtx = self.fs_abstraction.lock().expect("Locking Mutex failed");
                    let mut backend = mtx.get_mut();

                    if let Some(ref mut cur) = backend.get_mut(absent_path) {
                        let mut vec = cur.get_mut();
                        vec.clear();
                        vec.extend_from_slice(buf);
                        return Ok(());
                    }
                    let vec = Vec::from(buf);
                    backend.insert(absent_path.clone(), Cursor::new(vec));
                    return Ok(());
                },
            };
        }
    }

    #[derive(Debug)]
    pub struct InMemoryFileAbstraction {
        virtual_filesystem: Backend,
    }

    impl InMemoryFileAbstraction {

        pub fn new() -> InMemoryFileAbstraction {
            InMemoryFileAbstraction {
                virtual_filesystem: Arc::new(Mutex::new(RefCell::new(HashMap::new()))),
            }
        }

        pub fn backend(&self) -> &Backend {
            &self.virtual_filesystem
        }

    }

    impl FileAbstraction for InMemoryFileAbstraction {

        fn remove_file(&self, path: &PathBuf) -> Result<(), SE> {
            debug!("Removing: {:?}", path);
            self.backend()
                .lock()
                .expect("Locking Mutex failed")
                .get_mut()
                .remove(path)
                .map(|_| ())
                .ok_or(SEK::FileNotFound.into_error())
        }

        fn copy(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            debug!("Copying : {:?} -> {:?}", from, to);
            let mut mtx = self.backend().lock().expect("Locking Mutex failed");
            let mut backend = mtx.get_mut();

            let a = try!(backend.get(from).cloned().ok_or(SEK::FileNotFound.into_error()));
            backend.insert(to.clone(), a);
            debug!("Copying: {:?} -> {:?} worked", from, to);
            Ok(())
        }

        fn rename(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            debug!("Renaming: {:?} -> {:?}", from, to);
            let mut mtx = self.backend().lock().expect("Locking Mutex failed");
            let mut backend = mtx.get_mut();

            let a = try!(backend.get(from).cloned().ok_or(SEK::FileNotFound.into_error()));
            backend.insert(to.clone(), a);
            debug!("Renaming: {:?} -> {:?} worked", from, to);
            Ok(())
        }

        fn create_dir_all(&self, _: &PathBuf) -> Result<(), SE> {
            Ok(())
        }

        fn new_instance(&self, p: PathBuf) -> Box<FileAbstractionInstance> {
            Box::new(InMemoryFileAbstractionInstance::new(self.backend().clone(), p))
        }
    }

}

mod stdio {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::fmt::Error as FmtError;
    use std::fmt::Formatter;
    use std::io::Cursor;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::Mutex;

    use libimagerror::into::IntoError;
    use libimagerror::trace::*;

    use error::StoreErrorKind as SEK;
    use error::StoreError as SE;
    use super::FileAbstraction;
    use super::FileAbstractionInstance;
    use super::InMemoryFileAbstraction;

    // Because this is not exported in super::inmemory;
    type Backend = Arc<Mutex<RefCell<HashMap<PathBuf, Cursor<Vec<u8>>>>>>;

    mod mapper {
        use std::collections::HashMap;
        use std::io::Cursor;
        use std::io::{Read, Write};
        use std::path::PathBuf;
        use store::Result;

        pub trait Mapper {
            fn read_to_fs(&self, Box<Read>, &mut HashMap<PathBuf, Cursor<Vec<u8>>>)   -> Result<()>;
            fn fs_to_write(&self, &mut HashMap<PathBuf, Cursor<Vec<u8>>>, &mut Write) -> Result<()>;
        }
    }

    use self::mapper::Mapper;

    pub struct StdIoFileAbstraction<M: Mapper> {
        mapper: M,
        mem: InMemoryFileAbstraction,
        out: Box<Write>,
    }

    impl<M> Debug for StdIoFileAbstraction<M>
        where M: Mapper
    {
        fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
            write!(f, "StdIoFileAbstraction({:?}", self.mem)
        }
    }

    impl<M> StdIoFileAbstraction<M>
        where M: Mapper
    {

        pub fn new(in_stream: Box<Read>, out_stream: Box<Write>, mapper: M) -> Result<StdIoFileAbstraction<M>, SE> {
            let mem = InMemoryFileAbstraction::new();

            {
                let fill_res = match mem.backend().lock() {
                    Err(_) => Err(SEK::LockError.into_error()),
                    Ok(mut mtx) => mapper.read_to_fs(in_stream, mtx.get_mut())
                };
                let _ = try!(fill_res);
            }

            Ok(StdIoFileAbstraction {
                mapper: mapper,
                mem:    mem,
                out:    out_stream,
            })
        }

        pub fn backend(&self) -> &Backend {
            &self.mem.backend()
        }

    }

    impl<M> Drop for StdIoFileAbstraction<M>
        where M: Mapper
    {
        fn drop(&mut self) {
            let fill_res = match self.mem.backend().lock() {
                Err(_) => Err(SEK::LockError.into_error()),
                Ok(mut mtx) => self.mapper.fs_to_write(mtx.get_mut(), &mut *self.out)
            };

            // We can do nothing but end this here with a trace.
            // As this drop gets called when imag almost exits, there is no point in exit()ing here
            // again.
            let _ = fill_res.map_err_trace();
        }
    }

    impl<M: Mapper> FileAbstraction for StdIoFileAbstraction<M> {

        fn remove_file(&self, path: &PathBuf) -> Result<(), SE> {
            self.mem.remove_file(path)
        }

        fn copy(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            self.mem.copy(from, to)
        }

        fn rename(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
            self.mem.rename(from, to)
        }

        fn create_dir_all(&self, pb: &PathBuf) -> Result<(), SE> {
            self.mem.create_dir_all(pb)
        }

        fn new_instance(&self, p: PathBuf) -> Box<FileAbstractionInstance> {
            self.mem.new_instance(p)
        }
    }

}

#[cfg(test)]
mod test {
    use super::FileAbstractionInstance;
    use super::inmemory::InMemoryFileAbstraction;
    use super::inmemory::InMemoryFileAbstractionInstance;
    use std::path::PathBuf;

    #[test]
    fn lazy_file() {
        let fs = InMemoryFileAbstraction::new();

        let mut path = PathBuf::from("/tests");
        path.set_file_name("test1");
        let mut lf = InMemoryFileAbstractionInstance::new(fs.backend().clone(), path);
        lf.write_file_content(b"Hello World").unwrap();
        let bah = lf.get_file_content().unwrap();
        assert_eq!(bah, "Hello World");
    }

}
