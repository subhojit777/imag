use store::FileLockEntry;
use storeid::StoreId;
use hook::Hook;
use hook::result::HookResult;
use hook::accessor::{StoreIdAccessor, MutableHookDataAccessor, NonMutableHookDataAccessor};
use hook::accessor::HookDataAccessor as HDA;

#[derive(Debug)]
pub struct Aspect {
    name: String,
    hooks: Vec<Box<Hook>>,
}

impl Aspect {

    pub fn new(name: String) -> Aspect {
        Aspect {
            name: name,
            hooks: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn register_hook(&mut self, h: Box<Hook>) {
        self.hooks.push(h);
    }

}

impl StoreIdAccessor for Aspect {
    fn access(&self, id: &StoreId) -> HookResult<()> {
        let accessors : Vec<HDA> = self.hooks.iter().map(|h| h.accessor()).collect();
        if !accessors.iter().all(|a| match a { &HDA::StoreIdAccess(_)  => true, _ => false }) {
            unimplemented!()
        }

        for accessor in accessors {
            match accessor {
                HDA::StoreIdAccess(accessor) => try!(accessor.access(id)),
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

impl MutableHookDataAccessor for Aspect {
    fn access_mut(&self, fle: &mut FileLockEntry) -> HookResult<()> {
        let accessors : Vec<HDA> = self.hooks.iter().map(|h| h.accessor()).collect();
        if !accessors.iter().all(|a| match a { &HDA::MutableAccess(_)  => true, _ => false }) {
            unimplemented!()
        }

        for accessor in accessors {
            match accessor {
                HDA::MutableAccess(accessor) => try!(accessor.access_mut(fle)),
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

impl NonMutableHookDataAccessor for Aspect {
    fn access(&self, fle: &FileLockEntry) -> HookResult<()> {
        let accessors : Vec<HDA> = self.hooks.iter().map(|h| h.accessor()).collect();
        if !accessors.iter().all(|a| match a { &HDA::NonMutableAccess(_)  => true, _ => false }) {
            unimplemented!()
        }

        for accessor in accessors {
            match accessor {
                HDA::NonMutableAccess(accessor) => try!(accessor.access(fle)),
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

