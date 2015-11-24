use std::result::Result;

use super::ModuleError;
use storage::backend::{StorageBackend, StorageBackendError};

type CommandError = Result<ModuleError, StorageBackendError>;
type CommandResult = Result<(), Result<ModuleError, CommandError>>;

pub trait ExecutableCommand {
    fn get_callname() -> &'static str;
    fn exec(&self, rt: &Runtime, s: StorageBackend) -> CommandResult;
}
