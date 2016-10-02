//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::path::PathBuf;
use std::path::Path;
use std::fmt::{Debug, Formatter, Error as FmtError};
use std::result::Result as RResult;

use toml::Value;

use libimagerror::trace::trace_error;
use libimagstore::storeid::StoreId;
use libimagstore::hook::Hook;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::accessor::StoreIdAccessor;
use libimagutil::debug_result::*;

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::runtime::Runtime as GRuntime;

pub struct DeleteHook {
    storepath: PathBuf,

    runtime: GRuntime,

    position: HookPosition,
}

impl DeleteHook {

    pub fn new(storepath: PathBuf, p: HookPosition) -> DeleteHook {
        DeleteHook {
            runtime: GRuntime::new(&storepath),
            storepath: storepath,
            position: p,
        }
    }

}

impl Debug for DeleteHook {
    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "DeleteHook(storepath={:?}, repository={}, pos={:?}, cfg={:?})",
               self.storepath,
               (if self.runtime.has_repository() { "Some(_)" } else { "None" }),
               self.position,
               self.runtime.has_config())
    }
}


impl Hook for DeleteHook {

    fn name(&self) -> &'static str {
        "stdhook_git_delete"
    }

    /// Set the configuration of the hook. See
    /// `libimagstorestdhook::vcs::git::runtime::Runtime::set_config()`.
    ///
    /// This function traces the error (using `trace_error()`) that
    /// `libimagstorestdhook::vcs::git::runtime::Runtime::set_config()`
    /// returns, if any.
    fn set_config(&mut self, config: &Value) {
        if let Err(e) = self.runtime.set_config(config) {
            trace_error(&e);
        }
    }

}

impl HookDataAccessorProvider for DeleteHook {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl StoreIdAccessor for DeleteHook {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        use libimagerror::into::IntoError;
        use vcs::git::action::StoreAction;
        use vcs::git::config::commit_message;
        use vcs::git::error::MapIntoHookError;
        use vcs::git::util::fetch_index;
        use vcs::git::config::abort_on_repo_init_err;
        use vcs::git::config::is_enabled;
        use git2::{ADD_DEFAULT, STATUS_WT_DELETED, IndexMatchedPath};

        debug!("[GIT DELETE HOOK]: {:?}", id);

        let action = StoreAction::Delete;
        let cfg    = try!(self.runtime.config_value_or_err(&action));

        if !is_enabled(cfg) {
            return Ok(())
        }

        if !self.runtime.has_repository() {
            debug!("[GIT DELETE HOOK]: Runtime has no repository...");
            if try!(self.runtime.config_value_or_err(&action).map(|c| abort_on_repo_init_err(c))) {
                // Abort on repo init failure
                debug!("[GIT DELETE HOOK]: Config says we should abort if we have no repository");
                debug!("[GIT DELETE HOOK]: Returing Err(_)");
                return Err(GHEK::RepositoryInitError.into_error())
                    .map_err_into(GHEK::RepositoryError)
                    .map_into_hook_error()
            } else {
                debug!("[GIT DELETE HOOK]: Config says it is okay to not have a repository");
                debug!("[GIT DELETE HOOK]: Returing Ok(())");
                return Ok(())
            }
        }

        let _         = try!(self.runtime.ensure_cfg_branch_is_checked_out(&action));
        let repo      = try!(self.runtime.repository(&action));
        let mut index = try!(fetch_index(repo, &action));

        let signature = try!(
            repo.signature()
                .map_err_into(GHEK::MkSignature)
                .map_dbg_err_str("Failed to fetch signature")
                .map_dbg_str("[GIT DELETE HOOK]: Fetched signature object")
                .map_into_hook_error()
        );

        let head = try!(
            repo.head()
                .map_err_into(GHEK::HeadFetchError)
                .map_dbg_err_str("Failed to fetch HEAD")
                .map_dbg_str("[GIT DELETE HOOK]: Fetched HEAD")
                .map_into_hook_error()
        );

        let file_status = try!(
            repo
                .status_file(id.local())
                .map_dbg_err_str("Failed to fetch file status")
                .map_dbg_err(|e| format!("\t->  {:?}", e))
                .map_dbg_str("[GIT DELETE HOOK]: Fetched file status")
                .map_err_into(GHEK::RepositoryFileStatusError)
                .map_into_hook_error()
        );

        let cb = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
            debug!("[GIT DELETE HOOK]: Checking file status for: {}", path.display());
            if file_status.contains(STATUS_WT_DELETED) {
                debug!("[GIT DELETE HOOK]: File is deleted: {}", path.display());
                0
            } else {
                debug!("[GIT DELETE HOOK]: Ignoring file: {}", path.display());
                1
            }
        };

        try!(
            index.add_all(&[id.local()], ADD_DEFAULT, Some(cb as &mut IndexMatchedPath))
                .map_err_into(GHEK::RepositoryPathAddingError)
                .map_dbg_err_str("Failed to add to index")
                .map_dbg_str("[GIT DELETE HOOK]: Fetched index")
                .map_into_hook_error()
        );

        let tree_id = try!(
            index.write_tree()
                .map_err_into(GHEK::RepositoryIndexWritingError)
                .map_dbg_err_str("Failed to write tree")
                .map_dbg_str("[GIT DELETE HOOK]: Wrote index tree")
                .map_into_hook_error()
        );

        let mut parents = Vec::new();
        {
            let commit = try!(
                repo.find_commit(head.target().unwrap())
                    .map_err_into(GHEK::RepositoryParentFetchingError)
                    .map_dbg_err_str("Failed to find commit HEAD")
                    .map_dbg_str("[GIT DELETE HOOK]: Found commit HEAD")
                    .map_into_hook_error()
            );
            parents.push(commit);
        }

        // for converting from Vec<Commit> to Vec<&Commit>
        let parents = parents.iter().collect::<Vec<_>>();

        let tree = try!(
            repo.find_tree(tree_id)
                .map_err_into(GHEK::RepositoryParentFetchingError)
                .map_dbg_err_str("Failed to find tree")
                .map_dbg_str("[GIT DELETE HOOK]: Found tree for index")
                .map_into_hook_error()
        );

        let message = try!(commit_message(&repo, cfg, action, &id)
                .map_dbg_err_str("Failed to get commit message")
                .map_dbg_str("[GIT DELETE HOOK]: Got commit message"));

        try!(repo.commit(Some("HEAD"), &signature, &signature, &message, &tree, &parents)
            .map_dbg_str("Committed")
            .map_dbg_err_str("Failed to commit")
            .map_dbg_str("[GIT DELETE HOOK]: Committed")
            .map_err_into(GHEK::RepositoryCommittingError)
            .map_into_hook_error()
        );

        index.write()
            .map_err_into(GHEK::RepositoryIndexWritingError)
            .map_dbg_err_str("Failed to write tree")
            .map_dbg_str("[GIT DELETE HOOK]: Wrote index")
            .map_into_hook_error()
            .map(|_| ())
    }

}

