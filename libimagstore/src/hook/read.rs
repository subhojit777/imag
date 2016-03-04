use storeid::StoreId;
use store::FileLockEntry;
use hook::accessor::HookDataAccessorProvider;
use hook::result::HookResult;
use hook::Hook;

pub trait PreReadHook : Hook {
    fn pre_read(&self, &StoreId) -> HookResult<()>;
}

pub trait PostReadHook : Hook + HookDataAccessorProvider {
}

