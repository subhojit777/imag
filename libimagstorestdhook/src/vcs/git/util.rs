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

//! Utility functionality for integrating git hooks in the store
//!
//! Contains primitives to create a repository within the store path

use git2::{Repository, Index};

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::action::StoreAction;
use vcs::git::error::MapIntoHookError;

use libimagutil::debug_result::*;
use libimagstore::hook::error::HookError;

pub fn fetch_index(repo: &Repository, action: &StoreAction) -> Result<Index, HookError> {
    debug!("[GIT {} HOOK]: Getting Index", action.uppercase());
    repo.index()
        .map_dbg_err(|_| format!("[GIT {} HOOK]: Couldn't fetch Index", action.uppercase()))
        .map_dbg(|_| format!("[GIT {} HOOK]: Index object fetched", action.uppercase()))
        .map_err_into(GHEK::RepositoryIndexFetchingError)
        .map_into_hook_error()
}

