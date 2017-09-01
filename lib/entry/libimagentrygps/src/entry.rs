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

use result::Result;
use error::GPSErrorKind as GPSEK;
use error::MapErrInto;
use types::*;

use libimagstore::store::Entry;

use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;

pub trait GPSEntry {

    fn set_coordinates(&mut self, c: Coordinates) -> Result<()>;
    fn get_coordinates(&self) -> Result<Option<Coordinates>>;

}

impl GPSEntry for Entry {

    fn set_coordinates(&mut self, c: Coordinates) -> Result<()> {
        self.get_header_mut()
            .insert("gps.coordinates", c.into())
            .map(|_| ())
            .map_err_into(GPSEK::HeaderWriteError)
    }

    fn get_coordinates(&self) -> Result<Option<Coordinates>> {
        match self.get_header().read("gps.coordinates").map_err_into(GPSEK::HeaderWriteError) {
            Ok(Some(hdr)) => Coordinates::from_value(hdr).map(Some),
            Ok(None)      => Ok(None),
            Err(e)        => Err(e),
        }
    }

}

