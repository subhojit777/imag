use hook::result::HookResult;
use store::FileLockEntry;
use storeid::StoreId;

pub trait StoreIdAccessor : Send + Sync {
    fn access(&self, &StoreId) -> HookResult<()>;
}

pub trait MutableHookDataAccessor : Send + Sync {
    fn access_mut(&self, &mut FileLockEntry) -> HookResult<()>;
}

pub trait NonMutableHookDataAccessor : Send + Sync {
    fn access(&self, &FileLockEntry) -> HookResult<()>;
}

pub enum HookDataAccessor {
    StoreIdAccess(Box<StoreIdAccessor>),
    MutableAccess(Box<MutableHookDataAccessor>),
    NonMutableAccess(Box<NonMutableHookDataAccessor>),
}

pub trait HookDataAccessorProvider {
    fn accessor(&self) -> Box<HookDataAccessor>;
}


