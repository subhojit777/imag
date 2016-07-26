use std::path::PathBuf;
use std::fmt::{Debug, Formatter, Error as FmtError};
use std::result::Result as RResult;

use toml::Value;
use git2::{Reference as GitReference, Repository, Error as Git2Error};

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::error::HookError as HE;
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::error::CustomData as HECD;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;
use libimagerror::trace::trace_error;
use libimagerror::into::IntoError;

use vcs::git::result::Result;
use vcs::git::error::MapErrInto;
use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::GitHookError as GHE;
use vcs::git::runtime::Runtime;

pub struct CreateHook<'a> {
    storepath: &'a PathBuf,

    runtime: Runtime,

    position: HookPosition,
}

impl<'a> CreateHook<'a> {

    pub fn new(storepath: &'a PathBuf, p: HookPosition) -> CreateHook<'a> {
        CreateHook {
            storepath: storepath,
            runtime: Runtime::new(storepath),
            position: p,
        }
    }

}

impl<'a> Debug for CreateHook<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "CreateHook(storepath={:?}, repository={}, pos={:?}, cfg={:?}",
               self.storepath,
               (if self.runtime.has_repository() { "Some(_)" } else { "None" }),
               self.position,
               self.runtime.has_config())
    }
}

impl<'a> Hook for CreateHook<'a> {

    fn name(&self) -> &'static str {
        "stdhook_git_create"
    }

    fn set_config(&mut self, config: &Value) {
        if let Err(e) = self.runtime.set_config(config) {
            trace_error(&e);
        }
    }

}

impl<'a> HookDataAccessorProvider for CreateHook<'a> {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl<'a> StoreIdAccessor for CreateHook<'a> {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT CREATE HOOK]: {:?}", id);
        try!(self
             .runtime
             .ensure_cfg_branch_is_checked_out()
             .map_err(Box::new)
             .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e)));
        let repository = try!(self.runtime.repository());

        // Now to the create() hook action

        unimplemented!()

        Ok(())
    }

}

