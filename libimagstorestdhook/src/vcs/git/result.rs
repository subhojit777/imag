use std::result::Result as RResult;

use vcs::git::error::GitHookError;

pub type Result<T> = RResult<T, GitHookError>;
