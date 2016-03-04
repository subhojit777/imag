use storeid::StoreId;
use store::FileLockEntry;
use hook::accessor::HookDataAccessorProvider;
use hook::result::HookResult;
use hook::Hook;

pub trait PreDeleteHook : Hook {
    fn pre_delete(&self, &StoreId) -> HookResult<()>;
}

pub trait PostDeleteHook : Hook {
    fn post_delete(&self, &StoreId) -> HookResult<()>;
}

