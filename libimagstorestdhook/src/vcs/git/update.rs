use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;

#[derive(Debug)]
pub struct UpdateHook {
    position: HookPosition,
    config: Option<Value>,
}

impl UpdateHook {

    pub fn new(p: HookPosition) -> UpdateHook {
        UpdateHook {
            position: p,
            config: None,
        }
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

