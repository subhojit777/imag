use std::path::PathBuf;

use git2::{Repository, Signature};

use libimagerror::into::IntoError;
use libimagerror::trace::trace_error;

use vcs::git::result::Result;
use vcs::git::error::GitHookErrorKind as GHEK;
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
            .and_then(|n| author_email(cfg).map(|m| Person::new(n, m)))
            .and_then(|author| {
                committer_name(cfg)
                    .and_then(|n| committer_email(cfg).map(|m| (author, Person::new(n, m))))
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

}

