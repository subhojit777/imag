use std::result::Result as RResult;

use error::TagError;

pub type Result<T> = RResult<T, TagError>;

