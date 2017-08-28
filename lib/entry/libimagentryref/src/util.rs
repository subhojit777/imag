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

use error::RefErrorKind as REK;
use result::Result;

use libimagstore::store::Entry;
use libimagerror::into::IntoError;

use toml::Value;
use toml_query::read::TomlValueReadExt;

/// Creates a Hash from a PathBuf by making the PathBuf absolute and then running a hash
/// algorithm on it
pub fn hash_path(pb: &PathBuf) -> Result<String> {
    use crypto::sha1::Sha1;
    use crypto::digest::Digest;

    match pb.to_str() {
        Some(s) => {
            let mut hasher = Sha1::new();
            hasher.input_str(s);
            Ok(hasher.result_str())
        },
        None => return Err(REK::PathUTF8Error.into_error()),
    }
}

/// Read the reference from a file
pub fn read_reference(refentry: &Entry) -> Result<PathBuf> {
    match refentry.get_header().read("ref.path") {
        Ok(Some(&Value::String(ref s))) => Ok(PathBuf::from(s)),
        Ok(Some(_)) => Err(REK::HeaderTypeError.into_error()),
        Ok(None)    => Err(REK::HeaderFieldMissingError.into_error()),
        Err(e)      => Err(REK::StoreReadError.into_error_with_cause(Box::new(e))),
    }
}

