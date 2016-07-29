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
use vcs::git::runtime::Runtime;

pub struct CreateHook {
    storepath: PathBuf,

    runtime: Runtime,

    position: HookPosition,
}

impl CreateHook {

    pub fn new(storepath: PathBuf, p: HookPosition) -> CreateHook {
        CreateHook {
            runtime: Runtime::new(&storepath),
            storepath: storepath,
            position: p,
        }
    }

}

impl Debug for CreateHook {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "CreateHook(storepath={:?}, repository={}, pos={:?}, cfg={:?}",
               self.storepath,
               (if self.runtime.has_repository() { "Some(_)" } else { "None" }),
               self.position,
               self.runtime.has_config())
    }
}

impl Hook for CreateHook {

    fn name(&self) -> &'static str {
        "stdhook_git_create"
    }

    fn set_config(&mut self, config: &Value) {
        if let Err(e) = self.runtime.set_config(config) {
            trace_error(&e);
        }
    }

}

impl HookDataAccessorProvider for CreateHook {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl StoreIdAccessor for CreateHook {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        use vcs::git::action::StoreAction;
        use vcs::git::config::commit_message;

        debug!("[GIT CREATE HOOK]: {:?}", id);

        debug!("[GIT CREATE HOOK]: Ensuring branch checkout");
        try!(self
             .runtime
             .ensure_cfg_branch_is_checked_out()
             .map_err(Box::new)
             .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e)));
        debug!("[GIT CREATE HOOK]: Branch checked out");

        self.runtime
            .config_value_or_err()
            .map_err(|e| { debug!("[GIT CREATE HOOK]: Couldn't get Value object from config"); e })
            .and_then(|cfg| {
                debug!("[GIT CREATE HOOK]: Getting repository");
                self.runtime
                    .repository()
                    .map(|r| (r, cfg))
                    .map_err(|e| { debug!("[GIT CREATE HOOK]: Couldn't fetch Repository"); e })
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg)| {
                repo.signature()
                    .map(|s| (repo, cfg, s))
                    .map_err(|e| { debug!("[GIT CREATE HOOK]: Couldn't fetch Signature"); e })
                    .map_err_into(GHEK::RepositorySignatureFetchingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg, sig)| {
                repo.index()
                    .map(|idx| (repo, cfg, sig, idx))
                    .map_err(|e| { debug!("[GIT CREATE HOOK]: Couldn't fetch Index"); e })
                    .map_err_into(GHEK::RepositoryIndexFetchingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg, sig, mut idx)| {
                id.strip_prefix(&self.storepath)
                    .map_err_into(GHEK::StoreIdStripError)
                    .and_then(|id| idx.add_path(&id).map_err_into(GHEK::RepositoryPathAddingError))
                    .map(|_| (repo, cfg, sig, idx))
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| { debug!("[GIT CREATE HOOK]: Couldn't add Path: {:?}", e); e })
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg, sig, mut idx)| {
                idx.write_tree()
                    .map(|oid| (repo, cfg, sig, idx, oid))
                    .map_err(|e| { debug!("[GIT CREATE HOOK]: Couldn't write Tree"); e })
                    .map_err_into(GHEK::RepositoryTreeWritingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg, sig, idx, oid)| {
                repo.find_tree(oid)
                    .map(|tree| (repo, cfg, sig, idx, oid, tree))
                    .map_err(|e| { debug!("[GIT CREATE HOOK]: Couldn't find Tree"); e })
                    .map_err_into(GHEK::RepositoryTreeFindingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg, sig, idx, oid, tree)| {
                let cmtmsg = commit_message(cfg, StoreAction::Create);
                repo.find_commit(oid)
                    .map(|cmt| (repo, sig, tree, cmt, cmtmsg))
                    .map_err(|e| { debug!("[GIT CREATE HOOK]: Couldn't find Commit"); e })
                    .map_err_into(GHEK::RepositoryCommitFindingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, sig, tree, cmt, commitmsg)| {
                repo.commit(Some("HEAD"), &sig, &sig, &commitmsg[..], &tree, &[&cmt])
                    .map_err(|e| { debug!("[GIT CREATE HOOK]: Couldn't create Commit"); e })
                    .map_err_into(GHEK::RepositoryCommittingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .map(|_| ())
    }

}

