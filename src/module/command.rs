use std::result::Result;

use super::ModuleError;
use storage::backend::{StorageBackend, StorageBackendError};

pub type CommandError = Result<ModuleError, StorageBackendError>;
pub type CommandResult = Result<(), Result<ModuleError, CommandError>>;

pub trait ExecutableCommand {
    fn get_callname() -> &'static str;
    fn exec(&self, rt: &Runtime, s: StorageBackend) -> CommandResult;
}
