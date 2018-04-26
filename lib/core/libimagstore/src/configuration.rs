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

use toml::Value;

use store::Result;
use error::StoreError as SE;
use error::StoreErrorKind as SEK;

/// Checks whether the store configuration has a key "implicit-create" which maps to a boolean
/// value. If that key is present, the boolean is returned, otherwise false is returned.
pub fn config_implicit_store_create_allowed(config: &Option<Value>) -> Result<bool> {
    use toml_query::read::TomlValueReadTypeExt;

    let key = "store.implicit-create";

    if let Some(ref t) = *config {
        t.read_bool(key)?.ok_or_else(|| SE::from_kind(SEK::ConfigKeyMissingError(key)))
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use toml::de::from_str as toml_from_str;
    use configuration::*;

    #[test]
    fn test_implicit_store_create_allowed_no_toml() {
        assert!(!config_implicit_store_create_allowed(&None).unwrap());
    }

    #[test]
    fn test_implicit_store_create_allowed_toml_empty() {
        let config = toml_from_str("").unwrap();
        assert!(config_implicit_store_create_allowed(&Some(config)).is_err());
    }

    #[test]
    fn test_implicit_store_create_allowed_toml_false() {
        let config = toml_from_str(r#"
        [store]
            implicit-create = false
        "#).unwrap();

        assert!(!config_implicit_store_create_allowed(&Some(config)).unwrap());
    }

    #[test]
    fn test_implicit_store_create_allowed_toml_true() {
        let config = toml_from_str(r#"
        [store]
            implicit-create = true
        "#).unwrap();

        assert!(config_implicit_store_create_allowed(&Some(config)).unwrap());
    }

}

