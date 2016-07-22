use std::path::PathBuf;
use std::fmt::{Debug, Formatter, Error as FmtError};
use std::result::Result as RResult;

use toml::Value;
use git2::{Reference as GitReference, Repository, Error as Git2Error};

use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::error::HookError as HE;
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::error::CustomData as HECD;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;
use libimagerror::trace::trace_error;
use libimagerror::into::IntoError;

use vcs::git::result::Result;
use vcs::git::error::MapErrInto;
use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::GitHookError as GHE;
use vcs::git::config::ensure_branch;

pub struct CreateHook<'a> {
    storepath: &'a PathBuf,

    repository: Option<Repository>,

    position: HookPosition,
}

impl<'a> CreateHook<'a> {

    pub fn new(storepath: &'a PathBuf, p: HookPosition) -> CreateHook<'a> {
        let r = match Repository::open(storepath) {
            Ok(r) => Some(r),
            Err(e) => {
                trace_error(&e);
                None
            },
        };
        CreateHook {
            storepath: storepath,
            repository: r,
            position: p,
        }
    }

    fn repository(&self) -> HookResult<&Repository> {
        use vcs::git::config::abort_on_repo_init_err;

        match self.repository.as_ref() {
            Some(r) => Ok(r),
            None    => {
                debug!("Repository isn't initialized... creating error object now");
                let he = GHEK::MkRepo.into_error();
                let he = HE::new(HEK::HookExecutionError, Some(Box::new(he)));
                let custom = HECD::default().aborting(abort_on_repo_init_err(self.config.as_ref()));
                return Err(he.with_custom_data(custom));
            }
        }
    }

}

impl<'a> Debug for CreateHook<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "CreateHook(storepath={:?}, repository={}, pos={:?}, cfg={:?}",
               self.storepath,
               (if self.repository.is_some() { "Some(_)" } else { "None" }),
               self.position,
               self.runtime.has_config())
    }
}

impl<'a> Hook for CreateHook<'a> {

    fn name(&self) -> &'static str {
        "stdhook_git_create"
    }

    fn set_config(&mut self, config: &Value) {
        if let Err(e) = self.runtime.set_config(config) {
            trace_error(&e);
        }
    }

}

impl<'a> HookDataAccessorProvider for CreateHook<'a> {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl<'a> StoreIdAccessor for CreateHook<'a> {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT CREATE HOOK]: {:?}", id);
        let repository = try!(self.repository());
        let head       = try!(repository.head().map_err_into(GHEK::HeadFetchError)
             .map_err(|e| HEK::HookExecutionError.into_error_with_cause(Box::new(e))));

        if head.is_branch() {
            return GHEK::NotOnBranch.into_error().inside_of(HEK::HookExecutionError)
        }

        try!(checkout_branch(self.config.as_ref(), &head)
             .map_err(|e| HEK::HookExecutionError.into_error_with_cause(Box::new(e))));

        // Now to the create() hook action

        unimplemented!()

        Ok(())
    }

}

fn checkout_branch(config: Option<&Value>, head: &GitReference) -> Result<()> {
    // Check out appropriate branch ... or fail
    match ensure_branch(config) {
        Ok(Some(s)) => {
            match head.name().map(|name| name == s) {
                Some(b) => {
                    if b {
                        debug!("Branch already checked out.");
                        Ok(())
                    } else {
                        debug!("Branch not checked out.");
                        unimplemented!()
                    }
                },

                None => Err(GHEK::RepositoryBranchNameFetchingError.into_error())
                    .map_err_into(GHEK::RepositoryBranchError)
                    .map_err_into(GHEK::RepositoryError),
            }
        },
        Ok(None) => {
            debug!("No branch to checkout");
            Ok(())
        },

        Err(e) => Err(e).map_err_into(GHEK::RepositoryError),
    }
}

