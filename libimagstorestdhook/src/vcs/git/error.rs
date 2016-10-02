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

use git2::Error as Git2Error;

use libimagstore::hook::error::HookError as HE;
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::result::HookResult;

generate_error_module!(
    generate_error_types!(GitHookError, GitHookErrorKind,
        ConfigError => "Configuration Error",
        NoConfigError => "No Configuration",
        ConfigTypeError => "Configuration value type wrong",

        RepositoryError                   => "Error while interacting with git repository",
        RepositoryInitError               => "Error while loading the git repository",
        RepositoryBackendError            => "Error in the git library",
        RepositoryBranchError             => "Error while interacting with git branch(es)",
        RepositoryBranchNameFetchingError => "Error while fetching branch name",
        RepositoryWrongBranchError        => "Error because repository is on wrong branch",
        RepositoryIndexFetchingError      => "Error while fetching Repository Index",
        RepositoryIndexWritingError       => "Error while writing Repository Index",
        RepositoryPathAddingError         => "Error while adding Path to Index",
        RepositoryCommittingError         => "Error while committing",
        RepositoryParentFetchingError     => "Error while fetching parent of commit",

        HeadFetchError                    => "Error while getting HEAD",
        NotOnBranch                       => "No Branch is checked out",
        MkRepo                            => "Repository creation error",
        MkSignature                       => "Error while building Signature object",

        RepositoryFileStatusError         => "Error while getting file status",

        GitConfigFetchError               => "Error fetching git config",
        GitConfigEditorFetchError         => "Error fetching 'editor' from git config",
        EditorError                       => "Error while calling editor"
    );
);

impl GitHookError {

    pub fn inside_of<T>(self, h: HEK) -> HookResult<T> {
        Err(HE::new(h, Some(Box::new(self))))
    }

}

impl From<GitHookError> for HE {

    fn from(he: GitHookError) -> HE {
        HE::new(HEK::HookExecutionError, Some(Box::new(he)))
    }

}

impl From<Git2Error> for GitHookError {

    fn from(ge: Git2Error) -> GitHookError {
        GitHookError::new(GitHookErrorKind::RepositoryBackendError, Some(Box::new(ge)))
    }

}

pub trait MapIntoHookError<T> {
    fn map_into_hook_error(self) -> Result<T, HE>;
}

impl<T> MapIntoHookError<T> for Result<T, GitHookError> {

    fn map_into_hook_error(self) -> Result<T, HE> {
        self.map_err(|e| HE::new(HEK::HookExecutionError, Some(Box::new(e))))
    }

}

pub use self::error::GitHookError;
pub use self::error::GitHookErrorKind;
pub use self::error::MapErrInto;

