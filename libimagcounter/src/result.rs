use std::result::Result as RResult;

use error::CounterError;

pub type Result<T> = RResult<T, CounterError>;

