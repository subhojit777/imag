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

use error::TodoErrorKind as TEK;
use error::MapErrInto;
use result::Result;

use libimagstore::store::Entry;
use libimagerror::into::IntoError;

use uuid::Uuid;
use toml::Value;
use toml_query::read::TomlValueReadExt;

pub trait Task {
    fn get_uuid(&self) -> Result<Uuid>;
}

impl Task for Entry {
    fn get_uuid(&self) -> Result<Uuid> {
        match self.get_header().read("todo.uuid") {
            Ok(Some(&Value::String(ref uuid))) => {
                Uuid::parse_str(uuid).map_err_into(TEK::UuidParserError)
            },
            Ok(Some(_)) => Err(TEK::HeaderTypeError.into_error()),
            Ok(None)    => Err(TEK::HeaderFieldMissing.into_error()),
            Err(e)      => Err(e).map_err_into(TEK::StoreError),
        }
    }
}

