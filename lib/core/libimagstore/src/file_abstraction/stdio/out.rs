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

//! A StdIoFileAbstraction which does not read from stdin.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::ops::Deref;

use libimagerror::into::IntoError;
use libimagerror::trace::*;

use error::StoreErrorKind as SEK;
use error::StoreError as SE;
use super::FileAbstraction;
use super::FileAbstractionInstance;
use super::Drain;
use super::InMemoryFileAbstraction;
use store::Entry;

use super::mapper::Mapper;

// Because this is not exported in super::inmemory;
type Backend = Arc<Mutex<RefCell<HashMap<PathBuf, Entry>>>>;

pub struct StdoutFileAbstraction<W: Write, M: Mapper> {
    mapper: M,
    mem: InMemoryFileAbstraction,
    out: Rc<RefCell<W>>,
}

impl<W, M> StdoutFileAbstraction<W, M>
    where M: Mapper,
          W: Write
{

    pub fn new(out_stream: Rc<RefCell<W>>, mapper: M) -> Result<StdoutFileAbstraction<W, M>, SE> {
        Ok(StdoutFileAbstraction {
            mapper: mapper,
            mem:    InMemoryFileAbstraction::new(),
            out:    out_stream,
        })
    }

    pub fn backend(&self) -> &Backend {
        self.mem.backend()
    }

    fn backend_cloned(&self) -> Result<HashMap<PathBuf, Entry>, SE> {
        self.mem
            .backend()
            .lock()
            .map_err(|_| SEK::LockError.into_error())
            .map(|mtx| mtx.deref().borrow().clone())
    }

    pub fn mapper(&self) -> &M {
        &self.mapper
    }

}

impl<W, M> Debug for StdoutFileAbstraction<W, M>
    where M: Mapper,
          W: Write
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "StdoutFileAbstraction({:?}", self.mem)
    }
}

impl<W, M> Drop for StdoutFileAbstraction<W, M>
    where M: Mapper,
          W: Write
{
    fn drop(&mut self) {
        use std::ops::DerefMut;

        let fill_res = match self.mem.backend().lock() {
            Err(_) => Err(SEK::LockError.into_error()),
            Ok(mut mtx) => {
                self.mapper.fs_to_write(mtx.get_mut(), self.out.borrow_mut().deref_mut())
            },
        };

        // We can do nothing but end this here with a trace.
        // As this drop gets called when imag almost exits, there is no point in exit()ing here
        // again.
        let _ = fill_res.map_err_trace();
    }
}

impl<W: Write, M: Mapper> FileAbstraction for StdoutFileAbstraction<W, M> {

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

    fn drain(&self) -> Result<Drain, SE> {
        self.backend_cloned().map(Drain::new)
    }

    fn fill(&mut self, mut d: Drain) -> Result<(), SE> {
        debug!("Draining into : {:?}", self);
        let mut mtx = try!(self.backend().lock().map_err(|_| SEK::IoError.into_error()));
        let backend = mtx.get_mut();

        for (path, element) in d.iter() {
            debug!("Drain into {:?}: {:?}", self, path);
            backend.insert(path, element);
        }
        Ok(())
    }

}


