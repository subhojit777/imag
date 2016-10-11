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

use toml::Value;

use fs2::FileExt;
use std::fs::File;

use libimagstore::hook::Hook;
use libimagstore::hook::accessor::HookDataAccessor as HDA;
use libimagstore::hook::accessor::HookDataAccessorProvider;
use libimagstore::hook::accessor::StoreIdAccessor;
use libimagstore::hook::accessor::MutableHookDataAccessor;
use libimagstore::hook::accessor::NonMutableHookDataAccessor;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::error::{HookError, HookErrorKind};
use libimagstore::storeid::StoreId;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Entry;

mod error {
    generate_error_imports!();
    generate_error_types!(FlockError, FlockErrorKind,
        IOError                    => "IO Error",
        StoreIdPathBufConvertError => "Error while converting StoreId to PathBuf",
        FileOpenError              => "Error on File::open()",
        LockError                  => "Error while lock()ing",
        UnlockError                => "Error while unlock()ing"
    );
}
use self::error::FlockError as FE;
use self::error::FlockErrorKind as FEK;
use self::error::MapErrInto;

trait EntryFlock {
    fn lock(&self) -> Result<(), FE>;
    fn unlock(&self) -> Result<(), FE>;
}

fn open_file(id: StoreId) -> Result<File, FE> {
    id.into_pathbuf()
        .map_err_into(FEK::StoreIdPathBufConvertError)
        .and_then(|loc| {
            File::open(loc)
                .map_err_into(FEK::FileOpenError)
                .map_err_into(FEK::IOError)
        })
}

impl EntryFlock for Entry {

    fn lock(&self) -> Result<(), FE> {
        open_file(self.get_location().clone())
            .and_then(|file| {
                file.lock_exclusive()
                    .map_err_into(FEK::LockError)
                    .map_err_into(FEK::IOError)
            })
    }

    fn unlock(&self) -> Result<(), FE> {
        open_file(self.get_location().clone())
            .and_then(|file| {
                file.unlock()
                    .map_err_into(FEK::UnlockError)
                    .map_err_into(FEK::LockError)
                    .map_err_into(FEK::IOError)
            })
    }

}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Action {
    Lock,
    Unlock
}

fn action_to_str(a: &Action) -> &'static str {
    match *a {
        Action::Lock   => "lock",
        Action::Unlock => "unlock",
    }
}

#[derive(Debug, Clone)]
pub struct FlockUpdateHook {
    action: Action,
}

impl FlockUpdateHook {

    pub fn new(action: Action) -> FlockUpdateHook {
        FlockUpdateHook {
            action: action,
        }
    }

}

impl Hook for FlockUpdateHook {

    fn name(&self) -> &'static str {
        "stdhook_flock_update"
    }

    fn set_config(&mut self, _: &Value) {
        () // We are not configurable here.
    }

}

impl HookDataAccessorProvider for FlockUpdateHook {

    fn accessor(&self) -> HDA {
        HDA::StoreIdAccess(self)
    }

}

impl StoreIdAccessor for FlockUpdateHook {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[FLOCK HOOK][{}] {:?}", action_to_str(&self.action), id);
        Ok(())
    }

}

impl MutableHookDataAccessor for FlockUpdateHook {

    fn access_mut(&self, fle: &mut FileLockEntry) -> HookResult<()> {
        debug!("[FLOCK HOOK][{}] {:?}", action_to_str(&self.action), fle.get_location());
        fle.lock()
            .map_err(|e| HookError::new(HookErrorKind::HookExecutionError, Some(Box::new(e))))
            .map(|_| ())
    }

}

impl NonMutableHookDataAccessor for FlockUpdateHook {

    fn access(&self, fle: &FileLockEntry) -> HookResult<()> {
        debug!("[FLOCK HOOK][{}] {:?}", action_to_str(&self.action), fle.get_location());
        fle.unlock()
            .map_err(|e| HookError::new(HookErrorKind::HookExecutionError, Some(Box::new(e))))
            .map(|_| ())
    }

}

