use std::result::Result as RResult;

use error::BookmarkError;

pub type Result<T> = RResult<T, BookmarkError>;

