use storeid::StoreId;
use store::FileLockEntry;
use hook::accessor::HookDataAccessorProvider;
use hook::result::HookResult;
use hook::Hook;

pub trait PreRetrieveHook : Hook {
    fn pre_retrieve(&self, &StoreId) -> HookResult<()>;
}

pub trait PostRetrieveHook : Hook + HookDataAccessorProvider {
}

