//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

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

