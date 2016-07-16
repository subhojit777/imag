//! Utility functionality for integrating git hooks in the store
//!
//! Contains primitives to create a repository within the store path

use git2::Repository;
use git2::RepositoryInitOptions;

use libimagstore::store::Store;

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::result::Result;

pub fn mkrepo(store: &Store) -> Result<()> {
    let mut opts = RepositoryInitOptions::new();
    opts.bare(false);
    opts.no_reinit(true);
    opts.mkdir(false);
    opts.external_template(false);
    Repository::init_opts(store.path(), &opts)
        .map(|_| ())
        .map_err_into(GHEK::MkRepo)
}

pub fn hasrepo(store: &Store) -> bool {
    Repository::open(store.path()).is_ok()
}

