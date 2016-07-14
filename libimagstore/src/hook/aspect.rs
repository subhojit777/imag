use libimagerror::trace::trace_error;
use libimagutil::iter::FoldResult;

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
        let accessors : Vec<HDA> = self.hooks.iter().map(|h| h.accessor()).collect();
        if !accessors.iter().all(|a| is_match!(*a, HDA::StoreIdAccess(_))) {
            return Err(HE::new(HEK::AccessTypeViolation, None));
        }

        accessors.iter().fold_defresult(|accessor| {
            let res = match accessor {
                &HDA::StoreIdAccess(accessor) => accessor.access(id),
                _ => unreachable!(),
            };

            match res {
                Ok(res) => Ok(res),
                Err(e) => {
                    if !e.is_aborting() {
                        trace_error(&e);
                        // ignore error if it is not aborting, as we printed it already
                        Ok(())
                    } else {
                        Err(e)
                    }
                }
            }
        })
    }
}

impl MutableHookDataAccessor for Aspect {
    fn access_mut(&self, fle: &mut FileLockEntry) -> HookResult<()> {
        if !self.cfg.as_ref().map(|c| c.allow_mutable_hooks()).unwrap_or(false) {
            return Err(HE::new(HEK::MutableHooksNotAllowed, None));
        }

        let accessors : Vec<HDA> = self.hooks.iter().map(|h| h.accessor()).collect();

        fn is_file_accessor(a: &HDA) -> bool {
            is_match!(*a, HDA::MutableAccess(_) | HDA::NonMutableAccess(_))
        }

        if !accessors.iter().all(|a| is_file_accessor(a)) {
            return Err(HE::new(HEK::AccessTypeViolation, None));
        }

        // TODO: Naiive implementation.
        // More sophisticated version would check whether there are _chunks_ of
        // NonMutableAccess accessors and execute these chunks in parallel. We do not have
        // performance concerns yet, so this is okay.
        accessors.iter().fold_defresult(|accessor| {
            let res = match accessor {
                &HDA::MutableAccess(ref accessor)    => accessor.access_mut(fle),
                &HDA::NonMutableAccess(ref accessor) => accessor.access(fle),
                _ => unreachable!(),
            };

            match res {
                Ok(res) => Ok(res),
                Err(e) => {
                    if !e.is_aborting() {
                        trace_error(&e);
                        // ignore error if it is not aborting, as we printed it already
                        Ok(())
                    } else {
                        Err(e)
                    }
                }
            }
        })
    }
}

impl NonMutableHookDataAccessor for Aspect {
    fn access(&self, fle: &FileLockEntry) -> HookResult<()> {
        let accessors : Vec<HDA> = self.hooks.iter().map(|h| h.accessor()).collect();
        if !accessors.iter().all(|a| is_match!(*a, HDA::NonMutableAccess(_))) {
            return Err(HE::new(HEK::AccessTypeViolation, None));
        }

        accessors.iter().fold_defresult(|accessor| {
            let res = match accessor {
                &HDA::NonMutableAccess(accessor) => accessor.access(fle),
                _ => unreachable!(),
            };

            match res {
                Ok(res) => Ok(res),
                Err(e) => {
                    if !e.is_aborting() {
                        trace_error(&e);
                        // ignore error if it is not aborting, as we printed it already
                        Ok(())
                    } else {
                        Err(e)
                    }
                }
            }
        })
    }
}

