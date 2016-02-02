use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

#[derive(Debug)]
pub struct FieldPathElement {
    name: String
}

impl Display for FieldPathElement {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", self.name));
        Ok(())
    }

}


