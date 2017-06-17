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

//! The StdIo backend
//!
//! Sidenote: The name is "StdIo" because its main purpose is Stdin/Stdio, but it is abstracted over
//! Read/Write actually, so it is also possible to use this backend in other ways, too.
//!
//! So what is this about? This is a backend for the imag store which is created from stdin, by
//! piping contents into the store (via JSON or TOML) and piping the store contents (as JSON or
//! TOML) to stdout when the the backend is destructed.
//!
//! This is one of some components which make command-chaining in imag possible. With this, the
//! application does not have to know whether the store actually lives on the filesystem or just "in
//! memory".
//!
//! The backend contains a "Mapper" which defines how the contents get mapped into the in-memory
//! store representation: A JSON implementation or a TOML implementation are possible.
//!
//! In fact, a JSON implementation exists in the "json" submodule of this module.
//!

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

pub mod mapper;
use self::mapper::Mapper;

// Because this is not exported in super::inmemory;
type Backend = Arc<Mutex<RefCell<HashMap<PathBuf, Cursor<Vec<u8>>>>>>;

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


