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

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
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
use store::Entry;

pub mod mapper;
use self::mapper::Mapper;

// Because this is not exported in super::inmemory;
type Backend = Arc<Mutex<RefCell<HashMap<PathBuf, Entry>>>>;

pub struct StdIoFileAbstraction<W: Write, M: Mapper> {
    mapper: M,
    mem: InMemoryFileAbstraction,
    out: Rc<RefCell<W>>,
}

impl<W, M> Debug for StdIoFileAbstraction<W, M>
    where M: Mapper,
          W: Write
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "StdIoFileAbstraction({:?}", self.mem)
    }
}

impl<W, M> StdIoFileAbstraction<W, M>
    where M: Mapper,
          W: Write
{

    pub fn new<R: Read>(in_stream: &mut R, out_stream: Rc<RefCell<W>>, mapper: M) -> Result<StdIoFileAbstraction<W, M>, SE> {
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
        self.mem.backend()
    }

}

impl<W, M> Drop for StdIoFileAbstraction<W, M>
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

impl<W: Write, M: Mapper> FileAbstraction for StdIoFileAbstraction<W, M> {

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


