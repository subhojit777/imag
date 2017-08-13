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

use toml::Value;

use libimagerror::into::IntoError;

use store::Result;

/// Check whether the configuration is valid for the store
pub fn config_is_valid(config: &Option<Value>) -> Result<()> {
    use error::StoreErrorKind as SEK;

    if config.is_none() {
        return Ok(());
    }

    match *config {
        Some(Value::Table(_)) => Ok(()),
        _ => {
            warn!("Store config is no table");
            Err(SEK::ConfigTypeError.into_error())
        },
    }
}

/// Checks whether the store configuration has a key "implicit-create" which maps to a boolean
/// value. If that key is present, the boolean is returned, otherwise false is returned.
pub fn config_implicit_store_create_allowed(config: Option<&Value>) -> bool {
    config.map(|t| {
        match *t {
            Value::Table(ref t) => {
                match t.get("implicit-create") {
                    Some(&Value::Boolean(b)) => b,
                    Some(_) => {
                        warn!("Key 'implicit-create' does not contain a Boolean value");
                        false
                    }
                    None => {
                        warn!("Key 'implicit-create' in store configuration missing");
                        false
                    },
                }
            }
            _ => {
                warn!("Store configuration seems to be no Table");
                false
            },
        }
    }).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use toml::de::from_str as toml_from_str;
    use configuration::*;

    #[test]
    fn test_implicit_store_create_allowed_no_toml() {
        assert!(!config_implicit_store_create_allowed(None));
    }

    #[test]
    fn test_implicit_store_create_allowed_toml_empty() {
        let config = toml_from_str("").unwrap();
        assert!(!config_implicit_store_create_allowed(Some(config).as_ref()));
    }

    #[test]
    fn test_implicit_store_create_allowed_toml_false() {
        let config = toml_from_str(r#"
            implicit-create = false
        "#).unwrap();

        assert!(!config_implicit_store_create_allowed(Some(config).as_ref()));
    }

    #[test]
    fn test_implicit_store_create_allowed_toml_true() {
        let config = toml_from_str(r#"
            implicit-create = true
        "#).unwrap();

        assert!(config_implicit_store_create_allowed(Some(config).as_ref()));
    }

}

