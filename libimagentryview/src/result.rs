use std::result::Result as RResult;

use error::ViewError;

pub type Result<T> = RResult<T, ViewError>;
