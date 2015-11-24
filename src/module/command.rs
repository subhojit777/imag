use std::result::Result;

use clap::ArgMatches;

use runtime::Runtime;
use super::ModuleError;
use storage::backend::{StorageBackend, StorageBackendError};

pub type CommandError = Result<ModuleError, StorageBackendError>;
pub type CommandResult = Result<(), Result<ModuleError, CommandError>>;

pub struct CommandEnv<'a> {
    pub rt: &'a Runtime<'a>,
    pub matches: &'a ArgMatches<'a, 'a>,
    pub backend: StorageBackend
}

pub trait ExecutableCommand {
    fn get_callname() -> &'static str;
    fn exec(&self, env: CommandEnv) -> CommandResult;
}
