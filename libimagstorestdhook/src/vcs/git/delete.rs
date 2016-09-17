use std::path::PathBuf;

use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::runtime::Runtime as GRuntime;

#[derive(Debug)]
pub struct DeleteHook {
    storepath: PathBuf,

    runtime: GRuntime,

    position: HookPosition,
}

impl DeleteHook {

    pub fn new(storepath: PathBuf, p: HookPosition) -> DeleteHook {
        DeleteHook {
            runtime: GRuntime::new(&storepath),
            storepath: storepath,
            position: p,
        }
    }

}

impl Hook for DeleteHook {

    fn name(&self) -> &'static str {
        "stdhook_git_delete"
    }

    /// Set the configuration of the hook. See
    /// `libimagstorestdhook::vcs::git::runtime::Runtime::set_config()`.
    ///
    /// This function traces the error (using `trace_error()`) that
    /// `libimagstorestdhook::vcs::git::runtime::Runtime::set_config()`
    /// returns, if any.
    fn set_config(&mut self, config: &Value) {
        if let Err(e) = self.runtime.set_config(config) {
            trace_error(&e);
        }
    }

}

impl HookDataAccessorProvider for DeleteHook {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl StoreIdAccessor for DeleteHook {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT DELETE HOOK]: {:?}", id);
        Ok(())
    }

}

