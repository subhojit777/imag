use std::path::PathBuf;

use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;

#[derive(Debug)]
pub struct RetrieveHook<'a> {
    storepath: &'a PathBuf,

    position: HookPosition,
    config: Option<Value>,
}

impl<'a> RetrieveHook<'a> {

    pub fn new(storepath: &'a PathBuf, p: HookPosition) -> RetrieveHook<'a> {
        RetrieveHook {
            storepath: storepath,
            position: p,
            config: None,
        }
    }

}

impl<'a> Hook for RetrieveHook<'a> {

    fn name(&self) -> &'static str {
        "stdhook_git_retrieve"
    }

    fn set_config(&mut self, config: &Value) {
        self.config = Some(config.clone());
    }

}

impl<'a> HookDataAccessorProvider for RetrieveHook<'a> {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl<'a> StoreIdAccessor for RetrieveHook<'a> {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT RETRIEVE HOOK]: {:?}", id);
        Ok(())
    }

}

