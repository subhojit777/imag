use std::path::PathBuf;
use std::io::Read;

use error::RefErrorKind as REK;
use error::MapErrInto;

use crypto::sha1::Sha1;
use crypto::digest::Digest;

use result::Result;

/// The Hasher trait is used to implement custom hashing functions for the ref library.
/// This means that one can define how the hash of a reference is constructed from the content of
/// the file to ref to.
pub trait Hasher {

    fn hash_name(&self) -> &'static str;
    fn create_hash<R: Read>(&mut self, pb: &PathBuf, contents: &mut R) -> Result<String>;

}

pub struct DefaultHasher {
    hasher: Sha1,
}

impl DefaultHasher {

    pub fn new() -> DefaultHasher {
        DefaultHasher { hasher: Sha1::new() }
    }

}

impl Hasher for DefaultHasher {

    fn hash_name(&self) -> &'static str {
        "default"
    }

    fn create_hash<R: Read>(&mut self, _: &PathBuf, c: &mut R) -> Result<String> {
        let mut s = String::new();
        try!(c.read_to_string(&mut s).map_err_into(REK::UTF8Error).map_err_into(REK::IOError));
        self.hasher.input_str(&s[..]);
        Ok(self.hasher.result_str())
    }

}

