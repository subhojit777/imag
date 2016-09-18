use std::path::PathBuf;

use git2::{Index, Repository};
use toml::Value;

use libimagerror::into::IntoError;
use libimagerror::trace::trace_error;
use libimagstore::hook::error::CustomData;
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::result::HookResult;
use libimagutil::debug_result::*;

use vcs::git::action::StoreAction;
use vcs::git::result::Result;
use vcs::git::error::{MapErrInto, GitHookErrorKind as GHEK};

/// Runtime object for git hook implementations.
///
/// Contains some utility functionality to hold the repository and the configuration for the hooks.
pub struct Runtime {
    repository: Option<Repository>,
    config: Option<Value>,
}

impl Runtime {

    /// Build a `Runtime` object, pass the store path to build the `Repository` instance the
    /// `Runtime` has to contain.
    ///
    /// If the building of the `Repository` fails, this function `trace_error()`s the error and
    /// returns a `Runtime` object that does _not_ contain a `Repository`.
    pub fn new(storepath: &PathBuf) -> Runtime {
        Runtime {
            repository: match Repository::open(storepath) {
                Ok(r) => Some(r),
                Err(e) => {
                    trace_error(&e);
                    None
                },
            },

            config: None,
        }
    }

    /// Set the configuration for the `Runtime`. Always returns `Ok(())`.
    pub fn set_config(&mut self, cfg: &Value) -> Result<()> {
        self.config = Some(cfg.clone());
        Ok(())
    }

    /// Check whether the `Runtime` has a `Repository`
    pub fn has_repository(&self) -> bool {
        self.repository.is_some()
    }

    /// Check whether the `Runtime` has a configuration
    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    /// Get the the config value by reference or get an `Err()` which can be returned to the callee
    /// of the Hook.
    ///
    /// The `action` Argument is required in case of `Err()` so the error message can be build
    /// correctly.
    pub fn config_value_or_err(&self, action: &StoreAction) -> HookResult<&Value> {
        self.config
            .as_ref()
            .ok_or(GHEK::NoConfigError.into_error())
            .map_err_into(GHEK::ConfigError)
            .map_err(Box::new)
            .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e))
            .map_err(|mut e| e.with_custom_data(CustomData::default().aborting(false)))
            .map_dbg_err(|_| {
                format!("[GIT {} HOOK]: Couldn't get Value object from config", action.uppercase())
            })
    }

    /// Get the `Repository` object from the `Runtime` or an `Err()` that can be returned to the
    /// callee of the Hook.
    ///
    /// The `action` Argument is required in case of `Err()` so the error message can be build
    /// correctly.
    pub fn repository(&self, action: &StoreAction) -> HookResult<&Repository> {
        use vcs::git::error::MapIntoHookError;

        debug!("[GIT {} HOOK]: Getting repository", action.uppercase());
        self.repository
            .as_ref()
            .ok_or(GHEK::MkRepo.into_error())
            .map_err_into(GHEK::RepositoryError)
            .map_into_hook_error()
            .map_dbg_err(|_| format!("[GIT {} HOOK]: Couldn't fetch Repository", action.uppercase()))
            .map_dbg(|_| format!("[GIT {} HOOK]: Repository object fetched", action.uppercase()))
    }

    /// Ensure that the branch that is put in the configuration file is checked out, if any.
    pub fn ensure_cfg_branch_is_checked_out(&self, action: &StoreAction) -> HookResult<()> {
        use vcs::git::config::ensure_branch;

        debug!("[GIT CREATE HOOK]: Ensuring branch checkout");
        let head = try!(self
                        .repository(action)
                        .and_then(|r| {
                            debug!("Repository fetched, getting head");
                            r.head()
                                .map_dbg_err_str("Couldn't fetch HEAD")
                                .map_dbg_err(|e| format!("\tbecause = {:?}", e))
                                .map_err_into(GHEK::HeadFetchError)
                                .map_err(|e| e.into())
                        }));
        debug!("HEAD fetched");

        // TODO: Fail if not on branch? hmmh... I'm not sure
        if !head.is_branch() {
            debug!("HEAD is not a branch");
            return Err(GHEK::NotOnBranch.into_error().into());
        }
        debug!("HEAD is a branch");

        // Check out appropriate branch ... or fail
        match ensure_branch(self.config.as_ref()) {
            Ok(Some(s)) => {
                debug!("We have to ensure branch: {}", s);
                match head.name().map(|name| {
                    debug!("{} == {}", name, s);
                    name == s
                }) {
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
        .map_err(Box::new)
        .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e))
        .map_dbg_str("[GIT CREATE HOOK]: Branch checked out")
    }

    /// Check whether the WD is "dirty" - whether there is a diff to the repository
    /// This function returns false if there is no `Repository` object in the `Runtime`
    pub fn repo_is_dirty(&self, index: &Index) -> bool {
        match self.repository.as_ref() {
            Some(repo) => {
                repo.diff_index_to_workdir(Some(index), None)
                    .map_dbg_str("Fetched diff: Index <-> WD")
                    .map_dbg_err_str("Failed to fetch diff: Index <-> WD")
                    .map(|diff| diff.deltas().count() != 0)
                    .unwrap_or(false)
            },

            None => {
                debug!("No repository: Cannot fetch diff: Index <-> WD");
                false
            }
        }

    }

}

