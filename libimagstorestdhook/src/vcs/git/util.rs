//! Utility functionality for integrating git hooks in the store
//!
//! Contains primitives to create a repository within the store path

use git2::{Repository, Index};

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::runtime::Runtime as GRuntime;
use vcs::git::action::StoreAction;
use vcs::git::error::MapIntoHookError;

use libimagutil::debug_result::*;
use libimagstore::hook::error::HookError;

pub fn fetch_repo<'a>(runtime: &'a GRuntime, action: &StoreAction) -> Result<&'a Repository, HookError>
{
    debug!("[GIT {} HOOK]: Getting repository", action.uppercase());
    runtime
        .repository()
        .map_dbg_err(|_| format!("[GIT {} HOOK]: Couldn't fetch Repository", action.uppercase()))
        .map_dbg(|_| format!("[GIT {} HOOK]: Repository object fetched", action.uppercase()))
        .map_err_into(GHEK::RepositoryError)
        .map_into_hook_error()
}

pub fn fetch_index(repo: &Repository, action: &StoreAction) -> Result<Index, HookError> {
    debug!("[GIT {} HOOK]: Getting Index", action.uppercase());
    repo.index()
        .map_dbg_err(|_| format!("[GIT {} HOOK]: Couldn't fetch Index", action.uppercase()))
        .map_dbg(|_| format!("[GIT {} HOOK]: Index object fetched", action.uppercase()))
        .map_err_into(GHEK::RepositoryIndexFetchingError)
        .map_into_hook_error()
}

