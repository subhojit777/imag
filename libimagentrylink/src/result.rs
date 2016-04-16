use std::result::Result as RResult;

use error::LinkError;

pub type Result<T> = RResult<T, LinkError>;

