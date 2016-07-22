use libimagstore::hook::error::HookError as HE;
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::result::HookResult;

generate_error_module!(
    generate_error_types!(GitHookError, GitHookErrorKind,
        ConfigError => "Configuration Error",
        NoConfigError => "No Configuration",
        ConfigTypeError => "Configuration value type wrong",
        RuntimeInformationSetupError => "Couldn't setup runtime information for git hook",
        RepositoryError                   => "Error while interacting with git repository",
        RepositoryBranchError             => "Error while interacting with git branch(es)",
        RepositoryBranchNameFetchingError => "Error while fetching branch name",
        RepositorySignatureFetchingError  => "Error while fetching Authors/Committers signature",
        RepositoryIndexFetchingError      => "Error while fetching Repository Index",
        RepositoryPathAddingError         => "Error while adding Path to Index",
        RepositoryTreeWritingError        => "Error while writing repository tree",
        RepositoryTreeFindingError        => "Error while finding repository tree",
        RepositoryCommitFindingError      => "Error while finding commit",
        RepositoryCommittingError         => "Error while committing",
        HeadFetchError                    => "Error while getting HEAD",
        NotOnBranch                       => "No Branch is checked out",
        MkRepo => "Repository creation error",
        MkSignature => "Error while building Signature object"
    );
);

impl GitHookError {

    pub fn inside_of<T>(self, h: HEK) -> HookResult<T> {
        Err(HE::new(h, Some(Box::new(self))))
    }

}

pub use self::error::GitHookError;
pub use self::error::GitHookErrorKind;
pub use self::error::MapErrInto;

