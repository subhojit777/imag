use std::result::Result as RResult;

use error::EditError;

pub type Result<T> = RResult<T, EditError>;

