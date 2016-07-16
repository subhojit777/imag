use std::path::PathBuf;

use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;

#[derive(Debug)]
pub struct UpdateHook<'a> {
    storepath: &'a PathBuf,

    position: HookPosition,
    config: Option<Value>,
}

impl<'a> UpdateHook<'a> {

    pub fn new(storepath: &'a PathBuf, p: HookPosition) -> UpdateHook<'a> {
        UpdateHook {
            storepath: storepath,
            position: p,
            config: None,
        }
    }

}

impl<'a> Hook for UpdateHook<'a> {

    fn name(&self) -> &'static str {
        "stdhook_git_update"
    }

    fn set_config(&mut self, config: &Value) {
        self.config = Some(config.clone());
    }

}

impl<'a> HookDataAccessorProvider for UpdateHook<'a> {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl<'a> StoreIdAccessor for UpdateHook<'a> {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT UPDATE HOOK]: {:?}", id);
        Ok(())
    }

}

