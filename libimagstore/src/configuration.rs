use toml::Value;
use hook::position::HookPosition;

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
/// [[aspects.misc]]
/// parallel = true
/// [[aspects.encryption]]
/// parallel = false
/// [[aspects.version-control]]
/// parallel = false
///
/// [[hooks.gnupg]]
/// aspect = "encryption"
/// key = "0x123456789"
///
/// [[hooks.git]]
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
pub fn config_is_valid(config: &Value) -> bool {
    use std::collections::BTreeMap;

    fn has_key_with_map(v: &BTreeMap<String, Value>, key: &str) -> bool {
        v.get(key).map(|t| match t { &Value::Table(_) => true, _ => false }).unwrap_or(false)
    }

    fn has_key_with_string_ary(v: &BTreeMap<String, Value>, key: &str) -> bool {
        v.get(key)
            .map(|t| match t {
                &Value::Array(ref a) => a.iter().all(|elem| {
                    match elem {
                        &Value::String(_) => true,
                        _ => false,
                    }
                }),
                _ => false
            }).unwrap_or(false)
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
        -> bool
        where F: Fn(&Value) -> bool
    {
        store_config.get(section) // The store config has the section `section`
            .map(|section_table| {
                match section_table { // which is
                    &Value::Table(ref section_table) => // a table
                        section_table
                            .values() // which has values,
                            .all(|cfg| { // and all of these values
                                match cfg {
                                    &Value::Table(ref hook_config) => { // are tables
                                        hook_config.get(key) // with a key
                                            // fullfilling this constraint
                                            .map(|hook_aspect| f(&hook_aspect))
                                            .unwrap_or(false)
                                    },
                                    _ => false,
                                }
                            }),
                    _ => false,
                }
            })
            .unwrap_or(false)
    }

    match config {
        &Value::Table(ref t) => {
            has_key_with_string_ary(t, "pre-read-hook-aspects")        &&
            has_key_with_string_ary(t, "post-read-hook-aspects")       &&
            has_key_with_string_ary(t, "pre-create-hook-aspects")      &&
            has_key_with_string_ary(t, "post-create-hook-aspects")     &&
            has_key_with_string_ary(t, "pre-retrieve-hook-aspects")    &&
            has_key_with_string_ary(t, "post-retrieve-hook-aspects")   &&
            has_key_with_string_ary(t, "pre-update-hook-aspects")      &&
            has_key_with_string_ary(t, "post-update-hook-aspects")     &&
            has_key_with_string_ary(t, "pre-delete-hook-aspects")      &&
            has_key_with_string_ary(t, "post-delete-hook-aspects")     &&

            // The section "hooks" has maps which have a key "aspect" which has a value of type
            // String
            check_all_inner_maps_have_key_with(t, "hooks", "aspect", |asp| {
                match asp { &Value::String(_) => true, _ => false }
            }) &&

            // The section "aspects" has maps which have a key "parllel" which has a value of type
            // Boolean
            check_all_inner_maps_have_key_with(t, "aspects", "parallel", |asp| {
                match asp { &Value::Boolean(_) => true, _ => false, }
            })
        }
        _ => false,
    }
}

pub fn get_pre_read_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreRead, value)
}

pub fn get_post_read_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostRead, value)
}

pub fn get_pre_create_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreCreate, value)
}

pub fn get_post_create_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostCreate, value)
}

pub fn get_pre_retrieve_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreRetrieve, value)
}

pub fn get_post_retrieve_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostRetrieve, value)
}

pub fn get_pre_update_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreUpdate, value)
}

pub fn get_post_update_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostUpdate, value)
}

pub fn get_pre_delete_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreDelete, value)
}

pub fn get_post_delete_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostDelete, value)
}

fn get_aspect_names_for_aspect_position(position: HookPosition, value: &Value) -> Vec<String> {
    unimplemented!()
}
