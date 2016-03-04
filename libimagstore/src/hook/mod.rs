use std::fmt::Debug;

use toml::Value;

use self::error::HookError;
use store::FileLockEntry;

pub mod accessor;
pub mod create;
pub mod delete;
pub mod error;
pub mod read;
pub mod result;
pub mod retrieve;
pub mod update;

pub trait Hook : Debug + Send + Sync {
    fn set_config(&mut self, cfg: Value);
}

