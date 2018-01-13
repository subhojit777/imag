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
use error::RefError as RE;
use error::Result;

use libimagstore::store::Entry;

use toml_query::read::TomlValueReadTypeExt;

/// Creates a Hash from a PathBuf by making the PathBuf absolute and then running a hash
/// algorithm on it
pub fn hash_path(pb: &PathBuf) -> Result<String> {
    use crypto::sha1::Sha1;
    use crypto::digest::Digest;

    pb.to_str()
        .ok_or(RE::from_kind(REK::PathUTF8Error))
        .map(|s| {
            let mut hasher = Sha1::new();
            hasher.input_str(s);
            hasher.result_str()
         })
}

/// Read the reference from a file
pub fn read_reference(refentry: &Entry) -> Result<PathBuf> {
    refentry.get_header()
        .read_string("ref.path")?
        .ok_or(RE::from_kind(REK::HeaderFieldMissingError))
        .map(PathBuf::from)
}

