use std::result::Result as RResult;

use error::RefError;

pub type Result<T> = RResult<T, RefError>;

