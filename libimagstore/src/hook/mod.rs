use std::fmt::Debug;

use toml::Value;

pub mod accessor;
pub mod aspect;
pub mod error;
pub mod position;
pub mod result;

use hook::accessor::HookDataAccessorProvider;

pub trait Hook : HookDataAccessorProvider + Debug + Send + Sync {
    fn name(&self) -> &'static str;
    fn set_config(&mut self, cfg: &Value);
}

