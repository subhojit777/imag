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

mod fs;
mod inmemory;
mod stdio;

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

