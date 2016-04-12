use std::result::Result as RResult;

use error::InteractionError;

pub type Result<T> = RResult<T, InteractionError>;
