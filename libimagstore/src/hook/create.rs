use storeid::StoreId;
use store::FileLockEntry;
use hook::accessor::HookDataAccessorProvider;
use hook::result::HookResult;
use hook::Hook;

pub trait PreCreateHook : Hook {
    fn pre_create(&self, &StoreId) -> HookResult<()>;
}

pub trait PostCreateHook : Hook + HookDataAccessorProvider {
}

