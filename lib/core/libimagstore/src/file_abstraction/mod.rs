//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use std::path::PathBuf;
use std::fmt::Debug;
use std::collections::HashMap;
use std::sync::Arc;

use error::StoreError as SE;
use store::Entry;
use storeid::StoreId;

mod fs;
mod inmemory;
pub(crate) mod iter;

pub use self::fs::FSFileAbstraction;
pub use self::fs::FSFileAbstractionInstance;
pub use self::inmemory::InMemoryFileAbstraction;
pub use self::inmemory::InMemoryFileAbstractionInstance;
use self::iter::PathIterator;

/// An abstraction trait over filesystem actions
pub trait FileAbstraction : Debug {
    fn remove_file(&self, path: &PathBuf) -> Result<(), SE>;
    fn copy(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE>;
    fn rename(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE>;
    fn create_dir_all(&self, _: &PathBuf) -> Result<(), SE>;

    fn exists(&self, &PathBuf) -> Result<bool, SE>;
    fn is_file(&self, &PathBuf) -> Result<bool, SE>;

    fn new_instance(&self, p: PathBuf) -> Box<FileAbstractionInstance>;

    fn drain(&self) -> Result<Drain, SE>;
    fn fill<'a>(&'a mut self, d: Drain) -> Result<(), SE>;

    fn pathes_recursively(&self, basepath: PathBuf, storepath: PathBuf, backend: Arc<FileAbstraction>) -> Result<PathIterator, SE>;
}

/// An abstraction trait over actions on files
pub trait FileAbstractionInstance : Debug {

    /// Get the contents of the FileAbstractionInstance, as Entry object.
    ///
    /// The `StoreId` is passed because the backend does not know where the Entry lives, but the
    /// Entry type itself must be constructed with the id.
    fn get_file_content(&mut self, id: StoreId) -> Result<Entry, SE>;
    fn write_file_content(&mut self, buf: &Entry) -> Result<(), SE>;
}

pub struct Drain(HashMap<PathBuf, Entry>);

impl Drain {

    pub fn new(hm: HashMap<PathBuf, Entry>) -> Drain {
        Drain(hm)
    }

    pub fn empty() -> Drain {
        Drain::new(HashMap::new())
    }

    pub fn iter<'a>(&'a mut self) -> DrainIter<'a> {
        DrainIter(self.0.drain())
    }

}

pub struct DrainIter<'a>(::std::collections::hash_map::Drain<'a, PathBuf, Entry>);

impl<'a> Iterator for DrainIter<'a> {
    type Item = (PathBuf, Entry);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::FileAbstractionInstance;
    use super::inmemory::InMemoryFileAbstraction;
    use super::inmemory::InMemoryFileAbstractionInstance;
    use storeid::StoreId;
    use store::Entry;

    #[test]
    fn lazy_file() {
        let fs = InMemoryFileAbstraction::default();

        let mut path = PathBuf::from("tests");
        path.set_file_name("test1");
        let mut lf = InMemoryFileAbstractionInstance::new(fs.backend().clone(), path.clone());

        let loca = StoreId::new_baseless(path).unwrap();
        let file = Entry::from_str(loca.clone(), &format!(r#"---
[imag]
version = "{}"
---
Hello World"#, env!("CARGO_PKG_VERSION"))).unwrap();

        lf.write_file_content(&file).unwrap();
        let bah = lf.get_file_content(loca).unwrap();
        assert_eq!(bah.get_content(), "Hello World");
    }

    #[test]
    fn lazy_file_multiline() {
        let fs = InMemoryFileAbstraction::default();

        let mut path = PathBuf::from("tests");
        path.set_file_name("test1");
        let mut lf = InMemoryFileAbstractionInstance::new(fs.backend().clone(), path.clone());

        let loca = StoreId::new_baseless(path).unwrap();
        let file = Entry::from_str(loca.clone(), &format!(r#"---
[imag]
version = "{}"
---
Hello World
baz"#, env!("CARGO_PKG_VERSION"))).unwrap();

        lf.write_file_content(&file).unwrap();
        let bah = lf.get_file_content(loca).unwrap();
        assert_eq!(bah.get_content(), "Hello World\nbaz");
    }

    #[test]
    fn lazy_file_multiline_trailing_newlines() {
        let fs = InMemoryFileAbstraction::default();

        let mut path = PathBuf::from("tests");
        path.set_file_name("test1");
        let mut lf = InMemoryFileAbstractionInstance::new(fs.backend().clone(), path.clone());

        let loca = StoreId::new_baseless(path).unwrap();
        let file = Entry::from_str(loca.clone(), &format!(r#"---
[imag]
version = "{}"
---
Hello World
baz

"#, env!("CARGO_PKG_VERSION"))).unwrap();

        lf.write_file_content(&file).unwrap();
        let bah = lf.get_file_content(loca).unwrap();
        assert_eq!(bah.get_content(), "Hello World\nbaz\n\n");
    }

}

