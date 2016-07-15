use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;

#[derive(Debug)]
pub struct DeleteHook {
    position: HookPosition,
    config: Option<Value>,
}

impl DeleteHook {

    pub fn new(p: HookPosition) -> DeleteHook {
        DeleteHook {
            position: p,
            config: None,
        }
    }

}

impl Hook for DeleteHook {

    fn name(&self) -> &'static str {
        "stdhook_git_delete"
    }

    fn set_config(&mut self, config: &Value) {
        self.config = Some(config.clone());
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

