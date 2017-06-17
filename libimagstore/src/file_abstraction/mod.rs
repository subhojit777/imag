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

use std::path::PathBuf;
use std::fmt::Debug;

use error::StoreError as SE;


mod fs;
mod inmemory;
pub mod stdio;

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

