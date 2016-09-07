use std::path::PathBuf;
use std::fmt::{Debug, Formatter, Error as FmtError};
use std::result::Result as RResult;

use toml::Value;

use libimagerror::into::IntoError;
use libimagerror::trace::trace_error;
use libimagstore::hook::Hook;
use libimagstore::hook::accessor::StoreIdAccessor;
use libimagstore::hook::accessor::{HookDataAccessor, HookDataAccessorProvider};
use libimagstore::hook::error::HookError as HE;
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::position::HookPosition;
use libimagstore::hook::result::HookResult;
use libimagstore::storeid::StoreId;
use libimagutil::debug_result::*;

use vcs::git::error::GitHookError as GHE;
use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::error::MapIntoHookError;
use vcs::git::result::Result;
use vcs::git::runtime::Runtime as GRuntime;

pub struct UpdateHook {
    storepath: PathBuf,

    runtime: GRuntime,

    position: HookPosition,
}

impl UpdateHook {

    pub fn new(storepath: PathBuf, p: HookPosition) -> UpdateHook {
        UpdateHook {
            runtime: GRuntime::new(&storepath),
            storepath: storepath,
            position: p,
        }
    }

}

impl Debug for UpdateHook {
    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "UpdateHook(storepath={:?}, repository={}, pos={:?}, cfg={:?}",
               self.storepath,
               (if self.runtime.has_repository() { "Some(_)" } else { "None" }),
               self.position,
               self.runtime.has_config())
    }
}

impl Hook for UpdateHook {

    fn name(&self) -> &'static str {
        "stdhook_git_update"
    }

    fn set_config(&mut self, config: &Value) {
        if let Err(e) = self.runtime.set_config(config) {
            trace_error(&e);
        }
    }

}

impl HookDataAccessorProvider for UpdateHook {

    fn accessor(&self) -> HookDataAccessor {
        HookDataAccessor::StoreIdAccess(self)
    }
}

impl StoreIdAccessor for UpdateHook {

    /// The implementation of the UpdateHook
    ///
    /// # Scope
    ///
    /// This hook takes the git index and commits it either interactively or with a default message,
    /// if there is no configuration for an interactive commit.
    ///
    fn access(&self, id: &StoreId) -> HookResult<()> {
        use vcs::git::action::StoreAction;
        use vcs::git::config::commit_message;
        use vcs::git::error::MapIntoHookError;
        use vcs::git::util::fetch_repo;

        debug!("[GIT UPDATE HOOK]: {:?}", id);

        let cfg = try!(
            self.runtime
                .config_value_or_err()
                .map_dbg_err_str("[GIT UPDATE HOOK]: Couldn't get Value object from config")
        );

        let repo = try!(fetch_repo(&self.runtime,
            "[GIT UPDATE HOOK]: Getting repository",
            "[GIT UPDATE HOOK]: Couldn't fetch Repository",
            "[GIT UPDATE HOOK]: Repository object fetched")
        );

        let mut index = try!(
            repo
                .index()
                .map_err_into(GHEK::RepositoryIndexFetchingError)
                .map_into_hook_error()
        );

        let tree_id = try!(
            index.write_tree()
                .map_err_into(GHEK::RepositoryIndexWritingError)
                .map_into_hook_error()
        );

        let signature = try!(
            repo.signature()
                .map_err_into(GHEK::MkSignature)
                .map_into_hook_error()
        );

        let head = try!(
            repo.head()
                .map_err_into(GHEK::HeadFetchError)
                .map_into_hook_error()
        );

        let mut parents = Vec::new();
        {
            let commit = try!(
                repo.find_commit(head.target().unwrap())
                    .map_err_into(GHEK::RepositoryParentFetchingError)
                    .map_into_hook_error()
            );
            parents.push(commit);
        }

        // for converting from Vec<Commit> to Vec<&Commit>
        let parents = parents.iter().collect::<Vec<_>>();

        let tree = try!(
            repo.find_tree(tree_id)
                .map_err_into(GHEK::RepositoryParentFetchingError)
                .map_into_hook_error()
        );

        let message = try!(commit_message(cfg, StoreAction::Update));

        repo.commit(Some("HEAD"), &signature, &signature, &message, &tree, &parents)
            .map_err_into(GHEK::RepositoryCommittingError)
            .map_into_hook_error()
            .map(|_| ())

    }

}

