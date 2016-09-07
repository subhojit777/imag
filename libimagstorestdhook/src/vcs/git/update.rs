use std::path::PathBuf;

use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;

use vcs::git::runtime::Runtime as GRuntime;

pub struct UpdateHook {
    storepath: PathBuf,

    runtime: GRuntime,

    position: HookPosition,
}

impl UpdateHook {

    pub fn new(storepath: PathBuf, p: HookPosition) -> UpdateHook {
        UpdateHook {
            runtime: GRuntime::new(&storepath),
            storepath: storepath,
            position: p,
        }
    }

}

impl Debug for UpdateHook {
    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "UpdateHook(storepath={:?}, repository={}, pos={:?}, cfg={:?}",
               self.storepath,
               (if self.runtime.has_repository() { "Some(_)" } else { "None" }),
               self.position,
               self.runtime.has_config())
    }
}

impl Hook for UpdateHook {

    fn name(&self) -> &'static str {
        "stdhook_git_update"
    }

    fn set_config(&mut self, config: &Value) {
        self.config = Some(config.clone());
    }

}

impl HookDataAccessorProvider for UpdateHook {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl StoreIdAccessor for UpdateHook {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT UPDATE HOOK]: {:?}", id);
        Ok(())
    }

}

