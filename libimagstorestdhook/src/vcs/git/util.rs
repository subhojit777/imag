//! Utility functionality for integrating git hooks in the store
//!
//! Contains primitives to create a repository within the store path

use git2::Repository;

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::runtime::Runtime as GRuntime;
use vcs::git::action::StoreAction;
use vcs::git::error::MapIntoHookError;

use libimagutil::debug_result::*;
use libimagstore::hook::error::HookError;

pub fn fetch_repo<'a>(runtime: &'a GRuntime, action: &StoreAction) -> Result<&'a Repository, HookError>
{
    runtime
        .repository()
        .map_dbg_err_str(on_err_str)
        .map_err_into(GHEK::RepositoryError)
        .map_into_hook_error()
}

