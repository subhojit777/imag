use std::io::Read;
use std::path::PathBuf;
use std::result::Result as RResult;

use crypto::sha1::Sha1;
use crypto::digest::Digest;

use libimagerror::into::IntoError;

use hasher::Hasher;
use result::Result;
use error::RefErrorKind as REK;
use error::MapErrInto;

pub struct NBytesHasher {
    hasher: Sha1,
    n: usize,
}

impl NBytesHasher {

    pub fn new(n: usize) -> NBytesHasher {
        NBytesHasher {
            hasher: Sha1::new(),
            n: n,
        }
    }

}

impl Hasher for NBytesHasher {

    fn hash_name(&self) -> &'static str {
        "n-bytes-hasher"
    }

    fn create_hash<R: Read>(&mut self, _: &PathBuf, contents: &mut R) -> Result<String> {
        let s = contents
            .bytes()
            .take(self.n)
            .collect::<RResult<Vec<u8>, _>>()
            .map_err_into(REK::IOError)
            .and_then(|v| String::from_utf8(v).map_err_into(REK::IOError))
            .map_err(Box::new)
            .map_err(|e| REK::UTF8Error.into_error_with_cause(e))
            .map_err_into(REK::IOError);
        self.hasher.input_str(&try!(s)[..]);
        Ok(self.hasher.result_str())
    }

}

