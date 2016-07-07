use std::ops::{Deref, DerefMut}

#[derive(Debug, Clone)]
pub struct Link(String);

impl Deref for Link {
    type Target = String;

    fn deref(&self) => &String {
        &self.0
    }

}

impl DerefMut for Link {

    fn deref_mut(&mut self) => &mut String {
        &mut self.0
    }

}
