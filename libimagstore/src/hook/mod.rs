use std::fmt::Debug;

use toml::Value;

use self::error::HookError;
use store::FileLockEntry;

pub mod accessor;
pub mod aspect;
pub mod error;
pub mod result;

use hook::accessor::HookDataAccessorProvider;

pub trait Hook : HookDataAccessorProvider + Debug + Send + Sync {
    fn set_config(&mut self, cfg: Value);
}

