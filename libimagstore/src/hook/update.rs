use store::FileLockEntry;
use hook::accessor::HookDataAccessorProvider;
use hook::result::HookResult;
use hook::Hook;

pub trait PreUpdateHook : Hook {
    fn pre_update(&self, &FileLockEntry) -> HookResult<()>;
}

pub trait PostUpdateHook : Hook + HookDataAccessorProvider {
}

