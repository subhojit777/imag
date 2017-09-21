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

use error::Result;
use error::GPSErrorKind as GPSEK;
use error::ResultExt;
use types::*;

use libimagstore::store::Entry;

use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;
use toml_query::delete::TomlValueDeleteExt;

pub trait GPSEntry {

    fn set_coordinates(&mut self, c: Coordinates) -> Result<()>;
    fn get_coordinates(&self) -> Result<Option<Coordinates>>;

    /// Remove the coordinates from the entry
    ///
    /// # Returns
    ///
    /// The return type is a bit complicated, but that has a reason:
    ///
    /// The outer Result<_> is used for notifying a failure during the header read/write action.
    /// If the Option<_> is Some(_), the value was deleted.
    /// The inner Result<_> is used for parsing failures during the parsing of the deleted value.
    ///
    /// So:
    ///
    /// * Ok(Some(Ok(_))) if the coordinates were deleted, returning the deleted value
    /// * Ok(Some(Err(_))) if the coordinates were deleted, but the deleted value couldn't be parsed
    /// * Ok(None) if there were no coordinates to delete
    /// * Err(e) if the deleting failed
    ///
    fn remove_coordinates(&mut self) -> Result<Option<Result<Coordinates>>>;

}

impl GPSEntry for Entry {

    fn set_coordinates(&mut self, c: Coordinates) -> Result<()> {
        self.get_header_mut()
            .insert("gps.coordinates", c.into())
            .map(|_| ())
            .chain_err(|| GPSEK::HeaderWriteError)
    }

    fn get_coordinates(&self) -> Result<Option<Coordinates>> {
        match self.get_header().read("gps.coordinates").chain_err(|| GPSEK::HeaderWriteError) {
            Ok(Some(hdr)) => Coordinates::from_value(hdr).map(Some),
            Ok(None)      => Ok(None),
            Err(e)        => Err(e),
        }
    }

    fn remove_coordinates(&mut self) -> Result<Option<Result<Coordinates>>> {
        self.get_header_mut()
            .delete("gps.coordinates")
            .chain_err(|| GPSEK::HeaderWriteError)
            .map(|opt| opt.as_ref().map(Coordinates::from_value))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use libimagstore::store::Store;

    use entry::*;

    fn setup_logging() {
        use env_logger;
        let _ = env_logger::init().unwrap_or(());
    }

    fn get_store() -> Store {
        use libimagstore::file_abstraction::InMemoryFileAbstraction;
        let backend = Box::new(InMemoryFileAbstraction::new());
        Store::new_with_backend(PathBuf::from("/"), None, backend).unwrap()
    }

    #[test]
    fn test_set_gps() {
        setup_logging();

        let store = get_store();

        let mut entry = store.create(PathBuf::from("test_set_gps")).unwrap();

        let coordinates = Coordinates {
            latitude: GPSValue::new(0, 0, 0),
            longitude: GPSValue::new(0, 0, 0),
        };

        let res = entry.set_coordinates(coordinates);

        assert!(res.is_ok());
    }

    #[test]
    fn test_setget_gps() {
        setup_logging();

        let store = get_store();

        let mut entry = store.create(PathBuf::from("test_setget_gps")).unwrap();

        let coordinates = Coordinates {
            latitude: GPSValue::new(0, 0, 0),
            longitude: GPSValue::new(0, 0, 0),
        };

        let res = entry.set_coordinates(coordinates);
        assert!(res.is_ok());

        let coordinates = entry.get_coordinates();

        assert!(coordinates.is_ok());
        let coordinates = coordinates.unwrap();

        assert!(coordinates.is_some());
        let coordinates = coordinates.unwrap();

        assert_eq!(0, coordinates.longitude.degree);
        assert_eq!(0, coordinates.longitude.minutes);
        assert_eq!(0, coordinates.longitude.seconds);
        assert_eq!(0, coordinates.latitude.degree);
        assert_eq!(0, coordinates.latitude.minutes);
        assert_eq!(0, coordinates.latitude.seconds);
    }
}

