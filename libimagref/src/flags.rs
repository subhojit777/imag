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

use error::RefErrorKind as REK;
use result::Result;

pub struct RefFlags {
    content_hashing:       bool,
    permission_tracking:   bool,
}

impl RefFlags {

    /// Read the RefFlags from a TOML document
    ///
    /// Assumes that the whole TOML tree is passed. So this looks up `ref.flags` to get the flags.
    /// It assumes that this is a Map with Key = <name of the setting> and Value = boolean.
    pub fn read(v: &Value) -> Result<RefFlags> {
        fn get_field(v: &Value, key: &str) -> Result<bool> {
            match v.lookup(key) {
                Some(&Value::Boolean(b)) => Ok(b),
                Some(_) => Err(REK::HeaderTypeError.into()),
                None    => Err(REK::HeaderFieldMissingError.into()),
            }
        }

        Ok(RefFlags {
            content_hashing:     try!(get_field(v, "ref.flags.content_hashing")),
            permission_tracking: try!(get_field(v, "ref.flags.permission_tracking")),
        })
    }

    /// Alias for `RefFlags::content_hashing()`
    pub fn is_often_moving(self, b: bool) -> RefFlags {
        self.with_content_hashing(b)
    }

    pub fn with_content_hashing(mut self, b: bool) -> RefFlags {
        self.content_hashing = b;
        self
    }

    pub fn with_permission_tracking(mut self, b: bool) -> RefFlags {
        self.permission_tracking = b;
        self
    }


    pub fn get_content_hashing(&self) -> bool {
        self.content_hashing
    }

    pub fn get_permission_tracking(&self) -> bool {
        self.permission_tracking
    }

}

impl Into<Value> for RefFlags {

    /// Build a TOML::Value from this RefFlags object.
    ///
    /// Returns a Map which should be set in `ref.flags` in the header.
    fn into(self) -> Value {
        let mut btm = BTreeMap::new();
        btm.insert(String::from("content_hashing"),     Value::Boolean(self.content_hashing));
        btm.insert(String::from("permission_tracking"), Value::Boolean(self.permission_tracking));
        return Value::Table(btm)
    }

}

impl Default for RefFlags {

    fn default() -> RefFlags {
        RefFlags {
            content_hashing: false,
            permission_tracking: false,
        }
    }
}

