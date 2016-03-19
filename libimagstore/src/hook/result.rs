use hook::error::HookError;

pub type HookResult<T> = Result<T, HookError>;
