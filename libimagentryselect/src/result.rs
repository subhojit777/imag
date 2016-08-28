use std::result::Result as RResult;

use error::EntrySelectError;

pub type Result<T> = RResult<T, EntrySelectError>;

