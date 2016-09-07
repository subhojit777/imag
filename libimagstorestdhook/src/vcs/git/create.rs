use std::path::PathBuf;
use std::path::Path;
use std::fmt::{Debug, Formatter, Error as FmtError};
use std::result::Result as RResult;

use toml::Value;
use git2::{Reference as GitReference, Repository, Error as Git2Error};
use git2::{ADD_DEFAULT, STATUS_WT_NEW, STATUS_WT_MODIFIED, IndexMatchedPath};

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
use vcs::git::error::MapIntoHookError;
use vcs::git::error::MapErrInto;
use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::GitHookError as GHE;
use vcs::git::runtime::Runtime as GRuntime;

pub struct CreateHook {
    storepath: PathBuf,

    runtime: GRuntime,

    position: HookPosition,
}

impl CreateHook {

    pub fn new(storepath: PathBuf, p: HookPosition) -> CreateHook {
        CreateHook {
            runtime: GRuntime::new(&storepath),
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

    /// The implementation of the CreateHook
    ///
    /// # Scope
    ///
    /// What this function has to do is _adding_ the new entry to the git index.
    /// After that, the UpdateHook will take care of committing the changes or new file.
    ///
    fn access(&self, id: &StoreId) -> HookResult<()> {
        use vcs::git::action::StoreAction;
        use vcs::git::config::commit_message;
        use vcs::git::error::MapIntoHookError;
        use vcs::git::util::fetch_index;

        debug!("[GIT CREATE HOOK]: {:?}", id);

        let path = try!(
            id.clone()
            .into_pathbuf()
            .map_err_into(GHEK::StoreIdHandlingError)
            .map_into_hook_error()
        );

        let action    = StoreAction::Create;
        try!(self.runtime.ensure_cfg_branch_is_checked_out(&action));

        let cfg       = try!(self.runtime.config_value_or_err(&action));
        let repo      = try!(self.runtime.repository(&action));
        let mut index = try!(fetch_index(repo, &action));

        let file_status = try!(
            repo
                .status_file(&path)
                .map_err_into(GHEK::RepositoryFileStatusError)
                .map_into_hook_error()
        );

        let cb = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
            if file_status.contains(STATUS_WT_MODIFIED) ||
                file_status.contains(STATUS_WT_NEW) {

                debug!("[GIT CREATE HOOK]: File is new or modified: {}", path.display());
                0
            } else {
                debug!("[GIT CREATE HOOK]: Ignoring file: {}", path.display());
                1
            }
        };

        try!(
            index.add_all(&[path], ADD_DEFAULT, Some(cb as &mut IndexMatchedPath))
                .map_err_into(GHEK::RepositoryPathAddingError)
                .map_into_hook_error()
        );

        index
            .write()
            .map_err_into(GHEK::RepositoryIndexWritingError)
            .map_into_hook_error()
    }

}

