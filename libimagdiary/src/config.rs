use toml::Value;

use libimagrt::runtime::Runtime;

pub fn get_default_diary_name(rt: &Runtime) -> Option<String> {
    get_diary_config_section(rt)
        .and_then(|config| {
            match config.lookup("default_diary") {
                Some(&Value::String(ref s)) => Some(s.clone()),
                _ => None,
            }
        })
}

pub fn get_diary_config_section<'a>(rt: &'a Runtime) -> Option<&'a Value> {
    rt.config()
        .map(|config| config.config())
        .and_then(|config| config.lookup("diary"))
}
