use std::path::PathBuf;

use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;

#[derive(Debug)]
pub struct DeleteHook<'a> {
    storepath: &'a PathBuf,

    position: HookPosition,
    config: Option<Value>,
}

impl<'a> DeleteHook<'a> {

    pub fn new(storepath: &'a PathBuf, p: HookPosition) -> DeleteHook<'a> {
        DeleteHook {
            storepath: storepath,
            position: p,
            config: None,
        }
    }

}

impl<'a> Hook for DeleteHook<'a> {

    fn name(&self) -> &'static str {
        "stdhook_git_delete"
    }

    fn set_config(&mut self, config: &Value) {
        self.config = Some(config.clone());
    }

}

impl<'a> HookDataAccessorProvider for DeleteHook<'a> {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl<'a> StoreIdAccessor for DeleteHook<'a> {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT DELETE HOOK]: {:?}", id);
        Ok(())
    }

}

