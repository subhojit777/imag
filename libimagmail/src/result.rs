use std::result::Result as RResult;

use error::MailError;

pub type Result<T> = RResult<T, MailError>;

