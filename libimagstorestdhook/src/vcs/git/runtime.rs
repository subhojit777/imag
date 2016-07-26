use std::path::PathBuf;

use git2::{Repository, Signature};
use toml::Value;

use libimagerror::into::IntoError;
use libimagerror::trace::trace_error;
use libimagstore::hook::error::CustomData;
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::result::HookResult;

use vcs::git::result::Result;
use vcs::git::error::{MapErrInto, GitHookErrorKind as GHEK};

pub struct Runtime {
    repository: Option<Repository>,
    config: Option<Value>,
}

impl Runtime {

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

    pub fn set_config(&mut self, cfg: &Value) -> Result<()> {
        self.config = Some(cfg.clone());
        Ok(())
    }

    pub fn has_repository(&self) -> bool {
        self.repository.is_some()
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    pub fn config_value_or_err(&self) -> HookResult<&Value> {
        self.config
            .as_ref()
            .ok_or(GHEK::NoConfigError.into_error())
            .map_err_into(GHEK::ConfigError)
            .map_err(Box::new)
            .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e))
            .map_err(|mut e| e.with_custom_data(CustomData::default().aborting(false)))
    }

    pub fn repository(&self) -> Result<&Repository> {
        self.repository.as_ref().ok_or(GHEK::MkRepo.into_error())
    }

    pub fn ensure_cfg_branch_is_checked_out(&self) -> HookResult<()> {
        use vcs::git::config::ensure_branch;

        let head = try!(self
                        .repository()
                        .and_then(|r| {
                            r.head().map_err_into(GHEK::HeadFetchError)
                        })
                        .map_err(Box::new)
                        .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e)));

        // TODO: Fail if not on branch? hmmh... I'm not sure
        if head.is_branch() {
            return Err(GHEK::NotOnBranch.into_error())
                .map_err(Box::new)
                .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e));
        }

        // Check out appropriate branch ... or fail
        match ensure_branch(self.config.as_ref()) {
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
        .map_err(Box::new)
        .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e))
    }

}

