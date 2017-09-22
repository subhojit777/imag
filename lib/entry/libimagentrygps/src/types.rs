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

use std::collections::BTreeMap;

use toml::Value;

use error::GPSErrorKind as GPSEK;
use error::GPSError as GPSE;
use error::Result;

pub trait FromValue : Sized {
    fn from_value(v: &Value) -> Result<Self>;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GPSValue {
    pub degree:  i8,
    pub minutes: i8,
    pub seconds: i8
}

impl GPSValue {

    pub fn new(d: i8, m: i8, s: i8) -> GPSValue {
        GPSValue {
            degree:  d,
            minutes: m,
            seconds: s
        }
    }
}


impl Into<Value> for GPSValue {

    fn into(self) -> Value {
        let mut map = BTreeMap::new();
        let _ = map.insert("degree".to_owned(),  Value::Integer(self.degree as i64));
        let _ = map.insert("minutes".to_owned(), Value::Integer(self.minutes as i64));
        let _ = map.insert("seconds".to_owned(), Value::Integer(self.seconds as i64));
        Value::Table(map)
    }

}

impl FromValue for GPSValue {
    fn from_value(v: &Value) -> Result<Self> {
        match *v {
            Value::Table(ref map) => {
                Ok(GPSValue::new(
                    map.get("degree")
                        .ok_or_else(|| GPSE::from_kind(GPSEK::DegreeMissing))
                        .and_then(|v| match *v {
                            Value::Integer(i) => i64_to_i8(i),
                            _ => Err(GPSE::from_kind(GPSEK::HeaderTypeError)),
                        })?,

                    map
                        .get("minutes")
                        .ok_or_else(|| GPSE::from_kind(GPSEK::MinutesMissing))
                        .and_then(|v| match *v {
                            Value::Integer(i) => i64_to_i8(i),
                            _ => Err(GPSE::from_kind(GPSEK::HeaderTypeError)),
                        })?,

                    map
                        .get("seconds")
                        .ok_or_else(|| GPSE::from_kind(GPSEK::SecondsMissing))
                        .and_then(|v| match *v {
                            Value::Integer(i) => i64_to_i8(i),
                            _ => Err(GPSE::from_kind(GPSEK::HeaderTypeError)),
                        })?
                ))
            }
            _ => Err(GPSE::from_kind(GPSEK::TypeError))
        }
    }

}

/// Data-transfer type for transfering longitude-latitude-pairs
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Coordinates {
    pub longitude: GPSValue,
    pub latitude:  GPSValue,
}

impl Coordinates {
    pub fn new(long: GPSValue, lat: GPSValue) -> Coordinates {
        Coordinates {
            longitude: long,
            latitude:  lat,
        }
    }
}

impl Into<Value> for Coordinates {

    fn into(self) -> Value {
        let mut map = BTreeMap::new();
        let _ = map.insert("longitude".to_owned(), self.longitude.into());
        let _ = map.insert("latitude".to_owned(), self.latitude.into());
        Value::Table(map)
    }

}

impl FromValue for Coordinates {
    fn from_value(v: &Value) -> Result<Self> {
        match *v {
            Value::Table(ref map) => {
                Ok(Coordinates::new(
                    match map.get("longitude") {
                        Some(v) => GPSValue::from_value(v),
                        None    => Err(GPSE::from_kind(GPSEK::LongitudeMissing)),
                    }?,

                    match map.get("latitude") {
                        Some(v) => GPSValue::from_value(v),
                        None    => Err(GPSE::from_kind(GPSEK::LongitudeMissing)),
                    }?
                ))
            }
            _ => Err(GPSE::from_kind(GPSEK::TypeError))
        }
    }

}

/// Helper to convert a i64 to i8 or return an error if this doesn't work.
fn i64_to_i8(i: i64) -> Result<i8> {
    if i > (<i8>::max_value() as i64) {
        Err(GPSE::from_kind(GPSEK::NumberConversionError))
    } else {
        Ok(i as i8)
    }
}

