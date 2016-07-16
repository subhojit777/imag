use std::path::PathBuf;
use std::fmt::{Debug, Formatter, Error as FmtError};

use toml::Value;

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;

pub struct CreateHook<'a> {
    storepath: &'a PathBuf,

    position: HookPosition,
    config: Option<Value>,
}

impl<'a> CreateHook<'a> {

    pub fn new(storepath: &'a PathBuf, p: HookPosition) -> CreateHook<'a> {
        CreateHook {
            storepath: storepath,
            position: p,
            config: None,
        }
    }

}

impl<'a> Debug for CreateHook<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        write!(fmt, "CreateHook(storepath={:?}, repository={}, pos={:?}, cfg={:?}",
               self.storepath,
               (if self.repository.is_some() { "Some(_)" } else { "None" }),
               self.position,
               self.config)
    }
}

impl<'a> Hook for CreateHook<'a> {

    fn name(&self) -> &'static str {
        "stdhook_git_create"
    }

    fn set_config(&mut self, config: &Value) {
        self.config = Some(config.clone());
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
        Ok(())
    }

}

