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

use git2::Repository;
use toml::Value;

use libimagerror::into::IntoError;
use libimagerror::trace::MapErrTrace;
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
            repository: Repository::open(storepath).map_err_trace().ok(),
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
            .map_err(|e| e.with_custom_data(CustomData::default().aborting(false)))
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
        use vcs::git::config::do_checkout_ensure_branch;

        debug!("[GIT {} HOOK]: Ensuring branch checkout", action.uppercase());
        let head = try!(self
                        .repository(action)
                        .and_then(|r| {
                            debug!("[GIT {} HOOK]: Repository fetched, getting head", action.uppercase());
                            r.head()
                                .map_dbg_err_str("Couldn't fetch HEAD")
                                .map_dbg_err(|e| format!("\tbecause = {:?}", e))
                                .map_err_into(GHEK::HeadFetchError)
                                .map_err(|e| e.into())
                        }));
        debug!("[GIT {} HOOK]: HEAD fetched", action.uppercase());

        // TODO: Fail if not on branch? hmmh... I'm not sure
        if !head.is_branch() {
            debug!("[GIT {} HOOK]: HEAD is not a branch", action.uppercase());
            return Err(GHEK::NotOnBranch.into_error().into());
        }
        debug!("[GIT {} HOOK]: HEAD is a branch", action.uppercase());

        // Check out appropriate branch ... or fail
        match ensure_branch(self.config.as_ref()) {
            Ok(Some(s)) => {
                debug!("[GIT {} HOOK]: We have to ensure branch: {}", action.uppercase(), s);
                match head.name().map(|name| {
                    debug!("[GIT {} HOOK]: {} == {}", action.uppercase(), name, s);
                    name == s
                }) {
                    Some(b) => {
                        if b {
                            debug!("[GIT {} HOOK]: Branch already checked out.", action.uppercase());
                            Ok(())
                        } else {
                            debug!("[GIT {} HOOK]: Branch not checked out.", action.uppercase());

                            if !do_checkout_ensure_branch(self.config.as_ref()) {
                                Err(GHEK::RepositoryWrongBranchError.into_error())
                                    .map_err_into(GHEK::RepositoryError)
                            } else {
                                // Else try to check out the branch...
                                unimplemented!()
                            }
                        }
                    },

                    None => Err(GHEK::RepositoryBranchNameFetchingError.into_error())
                        .map_err_into(GHEK::RepositoryBranchError)
                        .map_err_into(GHEK::RepositoryError),
                }
            },
            Ok(None) => {
                debug!("[GIT {} HOOK]: No branch to checkout", action.uppercase());
                Ok(())
            },

            Err(e) => Err(e).map_err_into(GHEK::RepositoryError),
        }
        .map_err(Box::new)
        .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e))
        .map_dbg(|_| format!("[GIT {} HOOK]: Branch checked out", action.uppercase()))
    }

}

