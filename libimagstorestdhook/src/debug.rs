use toml::Value;

use libimagstore::hook::Hook;
use libimagstore::hook::accessor::HookDataAccessor;
use libimagstore::hook::accessor::HookDataAccessorProvider;
use libimagstore::hook::position::HookPosition;

use self::accessor::DebugHookAccessor as DHA;

#[derive(Debug)]
pub struct DebugHook {
    position: HookPosition,
    accessor: DHA,
}

impl DebugHook {

    pub fn new(pos: HookPosition) -> DebugHook {
        DebugHook {
            position: pos.clone(),
            accessor: DHA::new(pos),
        }
    }

}

impl Hook for DebugHook {

    fn name(&self) -> &'static str {
        "stdhook_debug"
    }

    fn set_config(&mut self, c: &Value) {
        debug!("Trying to set configuration in debug hook: {:?}", c);
        debug!("Ignoring configuration in debug hook, we don't need a config here");
    }

}

impl HookDataAccessorProvider for DebugHook {

    fn accessor(&self) -> HookDataAccessor {
        use libimagstore::hook::position::HookPosition as HP;
        use libimagstore::hook::accessor::HookDataAccessor as HDA;

        match self.position {
            HP::StoreUnload  |
            HP::PreCreate    |
            HP::PreRetrieve  |
            HP::PreDelete    |
            HP::PostDelete   => HDA::StoreIdAccess(&self.accessor),
            HP::PostCreate   |
            HP::PostRetrieve |
            HP::PreUpdate    |
            HP::PostUpdate   => HDA::MutableAccess(&self.accessor),
        }
    }

}

pub mod accessor {
    use std::ops::Deref;

    use libimagstore::storeid::StoreId;
    use libimagstore::store::FileLockEntry;
    use libimagstore::hook::result::HookResult;
    use libimagstore::hook::accessor::MutableHookDataAccessor;
    use libimagstore::hook::accessor::NonMutableHookDataAccessor;
    use libimagstore::hook::accessor::StoreIdAccessor;
    use libimagstore::hook::position::HookPosition;

    #[derive(Debug)]
    pub struct DebugHookAccessor {
        position: HookPosition,
    }

    impl DebugHookAccessor {

        pub fn new(position: HookPosition) -> DebugHookAccessor {
            DebugHookAccessor {
                position: position,
            }
        }

    }

    impl StoreIdAccessor for DebugHookAccessor {

        fn access(&self, id: &StoreId) -> HookResult<()> {
            debug!("[DEBUG HOOK]: {:?}", id);
            Ok(())
        }

    }

    impl MutableHookDataAccessor for DebugHookAccessor {

        fn access_mut(&self, fle: &mut FileLockEntry) -> HookResult<()> {
            debug!("[DEBUG HOOK] {:?}", fle.deref().deref());
            Ok(())
        }

    }

    impl NonMutableHookDataAccessor for DebugHookAccessor {

        fn access(&self, fle: &FileLockEntry) -> HookResult<()> {
            debug!("[DEBUG HOOK] {:?}", fle.deref().deref());
            Ok(())
        }

    }

}

