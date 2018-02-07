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

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::ops::Deref;
use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use error::StoreErrorKind as SEK;
use error::StoreError as SE;
use super::FileAbstraction;
use super::FileAbstractionInstance;
use super::Drain;
use super::InMemoryFileAbstraction;
use store::Entry;
use file_abstraction::iter::PathIterator;

pub mod mapper;
pub mod out;
use self::mapper::Mapper;
use self::out::StdoutFileAbstraction;

// Because this is not exported in super::inmemory;
type Backend = Arc<Mutex<RefCell<HashMap<PathBuf, Entry>>>>;

pub struct StdIoFileAbstraction<W: Write, M: Mapper>(StdoutFileAbstraction<W, M>);

impl<W, M> StdIoFileAbstraction<W, M>
    where M: Mapper,
          W: Write
{

    pub fn new<R: Read>(in_stream: &mut R, out_stream: Rc<RefCell<W>>, mapper: M) -> Result<StdIoFileAbstraction<W, M>, SE> {
        StdoutFileAbstraction::new(out_stream, mapper)
            .and_then(|out| {
                let _ = out.backend()
                     .lock()
                     .map_err(|_| SE::from_kind(SEK::LockError))
                     .map(|mut mtx| out.mapper().read_to_fs(in_stream, mtx.get_mut()))?;

                Ok(StdIoFileAbstraction(out))
            })
    }

    pub fn backend(&self) -> &Backend {
        self.0.backend()
    }

}

impl<W, M> Debug for StdIoFileAbstraction<W, M>
    where M: Mapper,
          W: Write
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "StdIoFileAbstraction({:?}", self.0)
    }
}

impl<W, M> Deref for StdIoFileAbstraction<W, M>
    where M: Mapper,
          W: Write
{
    type Target = StdoutFileAbstraction<W, M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// basically #[derive(FileAbstraction)]
impl<W: Write, M: Mapper> FileAbstraction for StdIoFileAbstraction<W, M> {

    fn remove_file(&self, path: &PathBuf) -> Result<(), SE> {
        self.0.remove_file(path)
    }

    fn copy(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
        self.0.copy(from, to)
    }

    fn rename(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
        self.0.rename(from, to)
    }

    fn create_dir_all(&self, pb: &PathBuf) -> Result<(), SE> {
        self.0.create_dir_all(pb)
    }

    fn new_instance(&self, p: PathBuf) -> Box<FileAbstractionInstance> {
        self.0.new_instance(p)
    }

    fn exists(&self, p: &PathBuf) -> Result<bool, SE> {
        self.0.exists(p)
    }

    fn is_file(&self, p: &PathBuf) -> Result<bool, SE> {
        self.0.is_file(p)
    }

    fn drain(&self) -> Result<Drain, SE> {
        self.0.drain()
    }

    fn fill(&mut self, d: Drain) -> Result<(), SE> {
        self.0.fill(d)
    }

    fn pathes_recursively(&self, basepath: PathBuf) -> Result<PathIterator, SE> {
        self.0.pathes_recursively(basepath)
    }
}

