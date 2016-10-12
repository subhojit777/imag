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
///
/// The passed `Value` _must be_ the `[store]` sub-tree of the configuration. Otherwise this will
/// fail.
///
/// It checks whether the configuration looks like the store wants it to be:
///
/// ```toml
/// [store]
/// pre-create-hook-aspects = [ "misc", "encryption", "version-control"]
///
/// [store.aspects.misc]
/// parallel = true
///
/// [store.aspects.encryption]
/// parallel = false
///
/// [store.aspects.version-control]
/// parallel = false
///
/// [store.hooks.gnupg]
/// aspect = "encryption"
/// key = "0x123456789"
///
/// [store.hooks.git]
/// aspect = "version-control"
/// ```
///
/// It checks:
///  * Whether all the maps are there (whether store, store.aspects, store.aspects.example are all
///  maps)
///  * Whether each aspect configuration has a "parallel = <Boolean>" setting
///  * Whether each hook congfiguration has a "aspect = <String>" setting
///
/// It does NOT check:
///  * Whether all aspects which are used in the hook configuration are also configured
///
/// No configuration is a valid configuration, as the store will use the most conservative settings
/// automatically. This has also performance impact, as all hooks run in no-parallel mode then.
/// You have been warned!
///
///
pub fn config_is_valid(config: &Option<Value>) -> Result<()> {
    use std::collections::BTreeMap;
    use error::StoreErrorKind as SEK;

    if config.is_none() {
        return Ok(());
    }

    /// Check whether the config has a key with a string array.
    /// The `key` is the key which is checked
    /// The `kind` is the error kind which is used as `cause` if there is an error, so we can
    /// indicate via error type which key is missing
    fn has_key_with_string_ary(v: &BTreeMap<String, Value>, key: &str,
                               kind: SEK) -> Result<()> {
        v.get(key)
            .ok_or_else(|| {
                warn!("Required key '{}' is not in store config", key);
                SEK::ConfigKeyMissingError.into_error_with_cause(Box::new(kind.into_error()))
            })
            .and_then(|t| match *t {
                Value::Array(ref a) => {
                    a.iter().fold(Ok(()), |acc, elem| {
                        acc.and_then(|_| {
                            if is_match!(*elem, Value::String(_)) {
                                Ok(())
                            } else {
                                let cause = Box::new(kind.into_error());
                                Err(SEK::ConfigTypeError.into_error_with_cause(cause))
                            }
                        })
                    })
                },
                _ => {
                    warn!("Key '{}' in store config should contain an array", key);
                    Err(SEK::ConfigTypeError.into_error_with_cause(Box::new(kind.into_error())))
                }
            })
    }

    /// Check that
    /// * the top-level configuration
    /// * is a table
    /// * where all entries of a key `section` (eg. "hooks" or "aspects")
    ///     * Are maps
    ///     * where each has a key `key` (eg. "aspect" or "parallel")
    ///     * which fullfills constraint `f` (typecheck)
    fn check_all_inner_maps_have_key_with<F>(store_config: &BTreeMap<String, Value>,
                                             section: &str,
                                             key: &str,
                                             f: F)
        -> Result<()>
        where F: Fn(&Value) -> bool
    {
        store_config.get(section) // The store config has the section `section`
            .ok_or_else(|| {
                warn!("Store config expects section '{}' to be present, but isn't.", section);
                SEK::ConfigKeyMissingError.into_error()
            })
            .and_then(|section_table| match *section_table { // which is
                Value::Table(ref section_table) => // a table
                    section_table.iter().fold(Ok(()), |acc, (inner_key, cfg)| {
                        acc.and_then(|_| {
                            match *cfg {
                                Value::Table(ref hook_config) => { // are tables
                                    // with a key
                                    let hook_aspect_is_valid = try!(hook_config.get(key)
                                        .map(|hook_aspect| f(&hook_aspect))
                                        .ok_or(SEK::ConfigKeyMissingError.into_error())
                                    );

                                    if !hook_aspect_is_valid {
                                        Err(SEK::ConfigTypeError.into_error())
                                    } else {
                                        Ok(())
                                    }
                                },
                                _ => {
                                    warn!("Store config expects '{}' to be in '{}.{}', but isn't.",
                                             key, section, inner_key);
                                    Err(SEK::ConfigKeyMissingError.into_error())
                                }
                            }
                        })
                    }),
                _ => {
                    warn!("Store config expects '{}' to be a Table, but isn't.", section);
                    Err(SEK::ConfigTypeError.into_error())
                }
            })
    }

    match *config {
        Some(Value::Table(ref t)) => {
            try!(has_key_with_string_ary(t, "store-unload-hook-aspects", SEK::ConfigKeyUnloadAspectsError));

            try!(has_key_with_string_ary(t, "pre-create-hook-aspects", SEK::ConfigKeyPreCreateAspectsError));
            try!(has_key_with_string_ary(t, "post-create-hook-aspects", SEK::ConfigKeyPostCreateAspectsError));
            try!(has_key_with_string_ary(t, "pre-retrieve-hook-aspects", SEK::ConfigKeyPreRetrieveAspectsError));
            try!(has_key_with_string_ary(t, "post-retrieve-hook-aspects", SEK::ConfigKeyPostRetrieveAspectsError));
            try!(has_key_with_string_ary(t, "pre-update-hook-aspects", SEK::ConfigKeyPreUpdateAspectsError));
            try!(has_key_with_string_ary(t, "post-update-hook-aspects", SEK::ConfigKeyPostUpdateAspectsError));
            try!(has_key_with_string_ary(t, "pre-delete-hook-aspects", SEK::ConfigKeyPreDeleteAspectsError));
            try!(has_key_with_string_ary(t, "post-delete-hook-aspects", SEK::ConfigKeyPostDeleteAspectsError));

            // The section "hooks" has maps which have a key "aspect" which has a value of type
            // String
            try!(check_all_inner_maps_have_key_with(t, "hooks", "aspect",
                                                    |asp| is_match!(asp, &Value::String(_))));

            // The section "aspects" has maps which have a key "parllel" which has a value of type
            // Boolean
            check_all_inner_maps_have_key_with(t, "aspects", "parallel",
                                               |asp| is_match!(asp, &Value::Boolean(_)))
        }
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

pub fn get_store_unload_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("store-unload-hook-aspects", value)
}

pub fn get_pre_create_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("pre-create-hook-aspects", value)
}

pub fn get_post_create_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("post-create-hook-aspects", value)
}

pub fn get_pre_retrieve_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("pre-retrieve-hook-aspects", value)
}

pub fn get_post_retrieve_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("post-retrieve-hook-aspects", value)
}

pub fn get_pre_update_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("pre-update-hook-aspects", value)
}

pub fn get_post_update_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("post-update-hook-aspects", value)
}

pub fn get_pre_delete_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("pre-delete-hook-aspects", value)
}

pub fn get_post_delete_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("post-delete-hook-aspects", value)
}

pub fn get_pre_move_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("pre-move-hook-aspects", value)
}

pub fn get_post_move_aspect_names(value: &Option<Value>) -> Vec<String> {
    get_aspect_names_for_aspect_position("post-move-hook-aspects", value)
}

#[derive(Debug)]
pub struct AspectConfig {
    parallel: bool,
    mutable_hooks: bool,
    config: Value,
}

impl AspectConfig {

    pub fn new(init: Value) -> AspectConfig {
        debug!("Trying to parse AspectConfig from: {:?}", init);
        let parallel = AspectConfig::is_parallel(&init);
        let muthooks = AspectConfig::allows_mutable_hooks(&init);
        AspectConfig {
            config: init,
            mutable_hooks: muthooks,
            parallel: parallel,
        }
    }

    fn is_parallel(init: &Value) -> bool {
        match *init {
            Value::Table(ref t) =>
                t.get("parallel")
                    .map_or(false, |value| {
                        match *value {
                            Value::Boolean(b) => b,
                            _ => false,
                        }
                    }),
            _ => false,
        }
    }

    fn allows_mutable_hooks(init: &Value) -> bool {
        match *init {
            Value::Table(ref t) =>
                t.get("mutable_hooks")
                    .map_or(false, |value| {
                        match *value {
                            Value::Boolean(b) => b,
                            _ => false,
                        }
                    }),
            _ => false,
        }
    }

    pub fn allow_mutable_hooks(&self) -> bool {
        self.mutable_hooks
    }

    /// Get the aspect configuration for an aspect.
    ///
    /// Pass the store configuration object, this searches in `[aspects][<aspect_name>]`.
    ///
    /// Returns `None` if one of the keys in the chain is not available
    pub fn get_for(v: &Option<Value>, a_name: String) -> Option<AspectConfig> {
        debug!("Get aspect configuration for {:?} from {:?}", a_name, v);
        let res = match *v {
            Some(Value::Table(ref tabl)) => {
                match tabl.get("aspects") {
                    Some(&Value::Table(ref tabl)) => {
                        tabl.get(&a_name[..]).map(|asp| AspectConfig::new(asp.clone()))
                    },

                    _ => None,
                }
            },
            _ => None,
        };
        debug!("Found aspect configuration for {:?}: {:?}", a_name, res);
        res
    }

}

fn get_aspect_names_for_aspect_position(config_name: &'static str, value: &Option<Value>) -> Vec<String> {
    let mut v = vec![];

    match *value {
        Some(Value::Table(ref t)) => {
            match t.get(config_name) {
                Some(&Value::Array(ref a)) => {
                    for elem in a {
                        match *elem {
                            Value::String(ref s) => v.push(s.clone()),
                            _ => warn!("Non-String in configuration, inside '{}'", config_name),
                        }
                    }
                },
                _ => warn!("'{}' configuration key should contain Array, does not", config_name),
            };
        },
        None => warn!("No store configuration, cannot get '{}'", config_name),
        _ => warn!("Configuration is not a table"),
    }
    v
}


