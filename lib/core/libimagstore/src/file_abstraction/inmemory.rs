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

use error::StoreError as SE;
use error::StoreErrorKind as SEK;
use std::collections::HashMap;
use std::sync::Mutex;
use std::cell::RefCell;
use std::sync::Arc;
use std::ops::Deref;

use super::FileAbstraction;
use super::FileAbstractionInstance;
use super::Drain;
use store::Entry;
use storeid::StoreId;
use file_abstraction::iter::PathIterator;

type Backend = Arc<Mutex<RefCell<HashMap<PathBuf, Entry>>>>;

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
    fn get_file_content(&mut self, _: StoreId) -> Result<Entry, SE> {
        debug!("Getting lazy file: {:?}", self);

        self.fs_abstraction
            .lock()
            .map_err(|_| SE::from_kind(SEK::LockError))
            .and_then(|mut mtx| {
                mtx.get_mut()
                    .get(&self.absent_path)
                    .cloned()
                    .ok_or_else(|| SE::from_kind(SEK::FileNotFound))
            })
    }

    fn write_file_content(&mut self, buf: &Entry) -> Result<(), SE> {
        match *self {
            InMemoryFileAbstractionInstance { ref absent_path, .. } => {
                let mut mtx = self.fs_abstraction.lock().expect("Locking Mutex failed");
                let backend = mtx.get_mut();
                let _ = backend.insert(absent_path.clone(), buf.clone());
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

    fn backend_cloned<'a>(&'a self) -> Result<HashMap<PathBuf, Entry>, SE> {
        self.virtual_filesystem
            .lock()
            .map_err(|_| SE::from_kind(SEK::LockError))
            .map(|mtx| mtx.deref().borrow().clone())
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
            .ok_or_else(|| SE::from_kind(SEK::FileNotFound))
    }

    fn copy(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
        debug!("Copying : {:?} -> {:?}", from, to);
        let mut mtx = self.backend().lock().expect("Locking Mutex failed");
        let backend = mtx.get_mut();

        let a = backend.get(from).cloned().ok_or_else(|| SE::from_kind(SEK::FileNotFound))?;
        backend.insert(to.clone(), a);
        debug!("Copying: {:?} -> {:?} worked", from, to);
        Ok(())
    }

    fn rename(&self, from: &PathBuf, to: &PathBuf) -> Result<(), SE> {
        debug!("Renaming: {:?} -> {:?}", from, to);
        let mut mtx = self.backend().lock().expect("Locking Mutex failed");
        let backend = mtx.get_mut();

        let a = backend.get(from).cloned().ok_or_else(|| SE::from_kind(SEK::FileNotFound))?;
        backend.insert(to.clone(), a);
        debug!("Renaming: {:?} -> {:?} worked", from, to);
        Ok(())
    }

    fn create_dir_all(&self, _: &PathBuf) -> Result<(), SE> {
        Ok(())
    }

    fn exists(&self, pb: &PathBuf) -> Result<bool, SE> {
        let mut mtx = self.backend().lock().expect("Locking Mutex failed");
        let backend = mtx.get_mut();

        Ok(backend.contains_key(pb))
    }

    fn is_file(&self, pb: &PathBuf) -> Result<bool, SE> {
        // Because we only store Entries in the memory-internal backend, we only have to check for
        // existance here, as if a path exists in the inmemory storage, it is always mapped to an
        // entry. hence it is always a path to a file
        self.exists(pb)
    }

    fn new_instance(&self, p: PathBuf) -> Box<FileAbstractionInstance> {
        Box::new(InMemoryFileAbstractionInstance::new(self.backend().clone(), p))
    }

    fn drain(&self) -> Result<Drain, SE> {
        self.backend_cloned().map(Drain::new)
    }

    fn fill<'a>(&'a mut self, mut d: Drain) -> Result<(), SE> {
        debug!("Draining into : {:?}", self);
        let mut mtx = self.backend().lock().map_err(|_| SEK::LockError)?;
        let backend = mtx.get_mut();

        for (path, element) in d.iter() {
            debug!("Drain into {:?}: {:?}", self, path);
            backend.insert(path, element);
        }

        Ok(())
    }

    fn pathes_recursively(&self, _basepath: PathBuf) -> Result<PathIterator, SE> {
        debug!("Getting all pathes");
        let keys : Vec<PathBuf> = self
            .backend()
            .lock()
            .map_err(|_| SE::from_kind(SEK::FileError))?
            .get_mut()
            .keys()
            .map(PathBuf::from)
            .collect();
        // we collect here as this happens only in tests and in memory anyways, so no problem

        Ok(PathIterator::new(Box::new(keys.into_iter())))
    }
}

