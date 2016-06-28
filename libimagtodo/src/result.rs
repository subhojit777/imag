use error::{TodoError, TodoErrorKind};

use std::result::Result as RResult;

pub type Result<T> = RResult<T, TodoError>;
