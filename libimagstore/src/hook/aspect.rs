use store::FileLockEntry;
use storeid::StoreId;
use hook::Hook;
use hook::result::HookResult;
use hook::accessor::{StoreIdAccessor, MutableHookDataAccessor, NonMutableHookDataAccessor};
use hook::accessor::HookDataAccessor as HDA;

use hook::error::HookError as HE;
use hook::error::HookErrorKind as HEK;
use configuration::AspectConfig;

#[derive(Debug)]
pub struct Aspect {
    cfg: Option<AspectConfig>,
    name: String,
    hooks: Vec<Box<Hook>>,
}

impl Aspect {

    pub fn new(name: String, cfg: Option<AspectConfig>) -> Aspect {
        Aspect {
            cfg: cfg,
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
        use crossbeam;

        let accessors : Vec<HDA> = self.hooks.iter().map(|h| h.accessor()).collect();
        if !accessors.iter().all(|a| match a { &HDA::StoreIdAccess(_)  => true, _ => false }) {
            return Err(HE::new(HEK::AccessTypeViolation, None));
        }

        let threads : Vec<HookResult<()>> = accessors
            .iter()
            .map(|accessor| {
                crossbeam::scope(|scope| {
                    scope.spawn(|| {
                        match accessor {
                            &HDA::StoreIdAccess(accessor) => accessor.access(id),
                            _ => unreachable!(),
                        }
                        .map_err(|e| ()) // TODO: We're losing the error cause here
                    })
                })
            })
            .map(|i| i.join().map_err(|_| HE::new(HEK::HookExecutionError, None)))
            .collect();

        threads
            .into_iter()
            .fold(Ok(()), |acc, elem| {
                acc.and_then(|a| {
                    elem.map(|_| a).map_err(|_| HE::new(HEK::HookExecutionError, None))
                })
            })
    }
}

impl MutableHookDataAccessor for Aspect {
    fn access_mut(&self, fle: &mut FileLockEntry) -> HookResult<()> {
        let accessors : Vec<HDA> = self.hooks.iter().map(|h| h.accessor()).collect();

        fn is_file_accessor(a: &HDA) -> bool {
            match a {
                &HDA::MutableAccess(_)  => true,
                &HDA::NonMutableAccess(_)  => true,
                _ => false,
            }
        }

        if !accessors.iter().all(|a| is_file_accessor(a)) {
            return Err(HE::new(HEK::AccessTypeViolation, None));
        }

        for accessor in accessors {
            match accessor {
                HDA::MutableAccess(accessor)    => try!(accessor.access_mut(fle)),

                // TODO: Naiive implementation.
                // More sophisticated version would check whether there are _chunks_ of
                // NonMutableAccess accessors and execute these chunks in parallel. We do not have
                // performance concerns yet, so this is okay.
                HDA::NonMutableAccess(accessor) => try!(accessor.access(fle)),
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

impl NonMutableHookDataAccessor for Aspect {
    fn access(&self, fle: &FileLockEntry) -> HookResult<()> {
        use crossbeam;

        let accessors : Vec<HDA> = self.hooks.iter().map(|h| h.accessor()).collect();
        if !accessors.iter().all(|a| match a { &HDA::NonMutableAccess(_)  => true, _ => false }) {
            return Err(HE::new(HEK::AccessTypeViolation, None));
        }

        let threads : Vec<HookResult<()>> = accessors
            .iter()
            .map(|accessor| {
                crossbeam::scope(|scope| {
                    scope.spawn(|| {
                        match accessor {
                            &HDA::NonMutableAccess(accessor) => accessor.access(fle),
                            _ => unreachable!(),
                        }
                        .map_err(|e| ()) // TODO: We're losing the error cause here
                    })
                })
            })
            .map(|i| i.join().map_err(|_| HE::new(HEK::HookExecutionError, None)))
            .collect();

        threads
            .into_iter()
            .fold(Ok(()), |acc, elem| {
                acc.and_then(|a| {
                    elem.map(|_| a).map_err(|_| HE::new(HEK::HookExecutionError, None))
                })
            })
    }
}

