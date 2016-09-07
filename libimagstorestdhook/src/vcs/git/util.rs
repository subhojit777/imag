//! Utility functionality for integrating git hooks in the store
//!
//! Contains primitives to create a repository within the store path

use git2::Repository;
use git2::RepositoryInitOptions;

use libimagstore::store::Store;

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::result::Result;

