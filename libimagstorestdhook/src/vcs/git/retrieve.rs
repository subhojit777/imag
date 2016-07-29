use std::path::PathBuf;

use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;

#[derive(Debug)]
pub struct RetrieveHook {
    storepath: PathBuf,

    position: HookPosition,
    config: Option<Value>,
}

impl RetrieveHook {

    pub fn new(storepath: PathBuf, p: HookPosition) -> RetrieveHook {
        RetrieveHook {
            storepath: storepath,
            position: p,
            config: None,
        }
    }

}

impl Hook for RetrieveHook {

    fn name(&self) -> &'static str {
        "stdhook_git_retrieve"
    }

    fn set_config(&mut self, config: &Value) {
        self.config = Some(config.clone());
    }

}

impl HookDataAccessorProvider for RetrieveHook {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl StoreIdAccessor for RetrieveHook {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT RETRIEVE HOOK]: {:?}", id);
        Ok(())
    }

}

