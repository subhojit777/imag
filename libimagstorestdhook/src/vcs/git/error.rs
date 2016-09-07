use git2::Error as Git2Error;

use libimagstore::hook::error::HookError as HE;
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::result::HookResult;

generate_error_module!(
    generate_error_types!(GitHookError, GitHookErrorKind,
        ConfigError => "Configuration Error",
        NoConfigError => "No Configuration",
        ConfigTypeError => "Configuration value type wrong",
        RuntimeInformationSetupError => "Couldn't setup runtime information for git hook",
        RepositoryBackendError            => "Error in the git library",
        RepositoryError                   => "Error while interacting with git repository",
        RepositoryBranchError             => "Error while interacting with git branch(es)",
        RepositoryBranchNameFetchingError => "Error while fetching branch name",
        RepositorySignatureFetchingError  => "Error while fetching Authors/Committers signature",
        RepositoryIndexFetchingError      => "Error while fetching Repository Index",
        RepositoryIndexWritingError       => "Error while writing Repository Index",
        RepositoryPathAddingError         => "Error while adding Path to Index",
        RepositoryTreeWritingError        => "Error while writing repository tree",
        RepositoryTreeFindingError        => "Error while finding repository tree",
        RepositoryCommitFindingError      => "Error while finding commit",
        RepositoryCommittingError         => "Error while committing",
        RepositoryHeadFetchingError       => "Error while fetching HEAD",
        RepositoryHeadTargetFetchingError => "Error while fetching target of HEAD",
        HeadFetchError                    => "Error while getting HEAD",
        NotOnBranch                       => "No Branch is checked out",
        MkRepo => "Repository creation error",
        MkSignature => "Error while building Signature object",
        StoreIdHandlingError => "Error handling the store id object",
        StoreIdStripError => "Couldn't strip prefix from StoreID object",

        RepositoryFileStatusError => "Error while getting file status"
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

pub use self::error::GitHookError;
pub use self::error::GitHookErrorKind;
pub use self::error::MapErrInto;

