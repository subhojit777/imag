use std::result::Result as RResult;

use error::MarkdownError;

pub type Result<T> = RResult<T, MarkdownError>;

