use std::fmt::Debug;

use hook::result::HookResult;
use store::FileLockEntry;
use storeid::StoreId;

pub trait StoreIdAccessor : Debug + Send + Sync {
    fn access(&self, &StoreId) -> HookResult<()>;
}

pub trait MutableHookDataAccessor : Debug + Send + Sync {
    fn access_mut(&self, &mut FileLockEntry) -> HookResult<()>;
}

pub trait NonMutableHookDataAccessor : Debug + Send + Sync {
    fn access(&self, &FileLockEntry) -> HookResult<()>;
}

#[derive(Debug)]
pub enum HookDataAccessor<'a> {
    StoreIdAccess(&'a StoreIdAccessor),
    MutableAccess(&'a MutableHookDataAccessor),
    NonMutableAccess(&'a NonMutableHookDataAccessor),
}

pub trait HookDataAccessorProvider {
    fn accessor(&self) -> HookDataAccessor;
}


