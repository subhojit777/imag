use std::result::Result as RResult;

use error::NoteError;

pub type Result<T> = RResult<T, NoteError>;

