//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use std::path::Path;

use libimagstore::storeid::StoreId;

use error::RefError;
use refstore::UniqueRefPathGenerator;

/// A base UniqueRefPathGenerator which must be overridden by the actual UniqueRefPathGenerator
/// which is provided by this crate
#[allow(dead_code)]
pub struct Base;

impl UniqueRefPathGenerator for Base {
    type Error = RefError;

    fn collection() -> &'static str {
        "ref"
    }

    fn unique_hash<A: AsRef<Path>>(_path: A) -> Result<String, Self::Error> {
        // This has to be overridden
        panic!("Not overridden base functionality. This is a BUG!")
    }

    fn postprocess_storeid(sid: StoreId) -> Result<StoreId, Self::Error> {
        Ok(sid) // default implementation
    }
}

