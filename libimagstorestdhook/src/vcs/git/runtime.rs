use std::path::PathBuf;

use git2::{Repository, Signature};
use toml::Value;

use libimagerror::into::IntoError;
use libimagerror::trace::trace_error;
use libimagstore::hook::error::CustomData;
use libimagstore::hook::error::HookErrorKind as HEK;

use vcs::git::result::Result;
use vcs::git::error::{MapErrInto, GitHookErrorKind as GHEK};
use vcs::git::config::{author_name, author_mail, committer_name, committer_mail};

struct Person<'a> {
    pub name: &'a str,
    pub mail: &'a str,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, mail: &'a str) -> Person<'a> {
        Person { name: name, mail: mail }
    }
}

pub struct Runtime<'a> {
    repository: Option<Repository>,
    author: Option<Person<'a>>,
    committer: Option<Person<'a>>,
}

impl<'a> Runtime<'a> {

    pub fn new(storepath: &PathBuf) -> Runtime<'a> {
        Runtime {
            repository: match Repository::open(storepath) {
                Ok(r) => Some(r),
                Err(e) => {
                    trace_error(&e);
                    None
                },
            },

            author: None,
            committer: None,
        }
    }

    pub fn configure(&mut self, config: &Value) -> Result<()> {
        author_name(cfg)
            .and_then(|n| author_mail(cfg).map(|m| Person::new(n, m)))
            .and_then(|author| {
                committer_name(cfg)
                    .and_then(|n| committer_mail(cfg).map(|m| (author, Person::new(n, m))))
            })
            .map(|(author, committer)| {
                self.author = Some(author);
                self.committer = Some(committer);
            })
    }

    pub fn new_committer_sig(&self) -> Option<Result<Signature>> {
        self.committer
            .as_ref()
            .map(|c| {
                Signature::now(c.name, c.mail)
                    .map_err(|e| GHEK::MkSignature.into_error_with_cause(Box::new(e)))
            })
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

