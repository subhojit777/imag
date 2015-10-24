use std::error::Error;

struct ApplicationError {
    str: String
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
    }

    fn cause(&self) -> Option<&Error> {
    }
}
