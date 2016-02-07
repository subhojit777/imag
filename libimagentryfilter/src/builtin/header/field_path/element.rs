use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

use toml::Value;

#[derive(Clone, Debug)]
pub struct FieldPathElement {
    name: String
}

impl FieldPathElement {

    pub fn new(name: String) -> FieldPathElement {
        FieldPathElement { name: name }
    }

    pub fn apply(&self, value: Value) -> Option<Value> {
        use std::str::FromStr;
        use std::ops::Index;

        match value {
            Value::Table(t) => {
                t.get(&self.name).map(|a| a.clone())
            },

            Value::Array(a) => {
                usize::from_str(&self.name[..])
                    .ok()
                    .and_then(|i| Some(a[i].clone()))
            },

            _ => None,
        }
    }

}

impl Display for FieldPathElement {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", self.name));
        Ok(())
    }

}


