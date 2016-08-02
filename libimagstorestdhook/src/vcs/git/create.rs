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
use libimagutil::debug_result::*;

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
            .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't get Value object from config")
            .and_then(|cfg| {
                debug!("[GIT CREATE HOOK]: Getting repository");
                self.runtime
                    .repository()
                    .map(|r| (r, cfg))
                    .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't fetch Repository")
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg)| {
                repo.signature()
                    .map(|s| (repo, cfg, s))
                    .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't fetch Signature")
                    .map_err_into(GHEK::RepositorySignatureFetchingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg, sig)| {
                repo.index()
                    .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't fetch Index")
                    .and_then(|mut idx| idx.write_tree().map(|t| (idx, t)))
                    .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't write Tree")
                    .and_then(|(idx, tree_id)| repo.find_tree(tree_id).map(|t| (idx, t)))
                    .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't find Tree")
                    .map(|(idx, tree)| (repo, cfg, sig, idx, tree))
                    .map_err_into(GHEK::RepositoryIndexFetchingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg, sig, mut idx, tree)| {
                idx.add_path(id)
                    .map(|_| (repo, cfg, sig, idx, tree))
                    .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't add Path")
                    .map_dbg_err(|_| format!("\tpath = {:?}", id))
                    .map_dbg_err(|e| format!("\terr  = {:?}", e))
                    .map_err_into(GHEK::RepositoryPathAddingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .and_then(|(repo, cfg, sig, idx, tree)| {
                repo.head()
                    .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't fetch HEAD")
                    .map_err_into(GHEK::RepositoryHeadFetchingError)
                    .map(|h| h.target())
                    .and_then(|oid| {
                        match oid {
                            Some(oid) => {
                                repo.find_commit(oid)
                                    .map(|c| Some(c))
                                    .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't find commit")
                                    .map_dbg_err(|_| format!("\toid = {:?}", oid))
                                    .map_err_into(GHEK::RepositoryCommitFindingError)
                            },
                            None => Ok(None),
                        }
                    })
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
                    .map(|parent| (repo, cfg, sig, idx, tree, parent))
            })
            .and_then(|(repo, cfg, sig, idx, tree, opt_parent)| {
                let (msg, parents) = match opt_parent {
                    None    => (String::from("Initial commit"), vec![]),
                    Some(p) => (commit_message(&cfg, StoreAction::Create), vec![p]),
                };

                let parents = parents.iter().collect::<Vec<_>>();
                repo.commit(Some("HEAD"), &sig, &sig, &msg[..], &tree, &parents)
                    .map_dbg_err_str("[GIT CREATE HOOK]: Couldn't commit")
                    .map_err_into(GHEK::RepositoryCommittingError)
                    .map_err_into(GHEK::RepositoryError)
                    .map_err(|e| e.into())
            })
            .map(|_| ())
    }

}

