use std::path::PathBuf;
use std::fmt::{Debug, Formatter, Error as FmtError};
use std::result::Result as RResult;

use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;
use libimagerror::trace::trace_error;

use vcs::git::runtime::Runtime as GRuntime;

pub struct CreateHook {
    storepath: PathBuf,

    runtime: GRuntime,

    position: HookPosition,
}

impl CreateHook {

    pub fn new(storepath: PathBuf, p: HookPosition) -> CreateHook {
        CreateHook {
            runtime: GRuntime::new(&storepath),
            storepath: storepath,
            position: p,
        }
    }

}

impl Debug for CreateHook {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "CreateHook(storepath={:?}, repository={}, pos={:?}, cfg={:?}",
               self.storepath,
               (if self.runtime.has_repository() { "Some(_)" } else { "None" }),
               self.position,
               self.runtime.has_config())
    }
}

impl Hook for CreateHook {

    fn name(&self) -> &'static str {
        "stdhook_git_create"
    }

    fn set_config(&mut self, config: &Value) {
        if let Err(e) = self.runtime.set_config(config) {
            trace_error(&e);
        }
    }

}

impl HookDataAccessorProvider for CreateHook {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl StoreIdAccessor for CreateHook {

    /// The implementation of the CreateHook
    ///
    /// # Scope
    ///
    /// What this function has to do is _adding_ the new entry to the git index.
    /// After that, the UpdateHook will take care of committing the changes or new file.
    ///
    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT CREATE HOOK]: {:?}", id);
        debug!("[GIT CREATE HOOK]: Doing nothing as Store::create() is lazy and does not write to disk");
        Ok(())
    }

}

