use std::result::Result as RResult;

use error::ListError;

pub type Result<T> = RResult<T, ListError>;

