use std::result::Result;

use clap::ArgMatches;

use runtime::Runtime;
use super::ModuleError;
use storage::backend::{StorageBackend, StorageBackendError};

pub type CommandError = Result<ModuleError, StorageBackendError>;
pub type CommandResult = Result<(), Result<ModuleError, CommandError>>;

pub trait ExecutableCommand {
    fn get_callname() -> &'static str;
    fn exec(&self,
            rt: &Runtime,
            matches: &ArgMatches<'a, 'a>,
            s: StorageBackend) -> CommandResult;
}
